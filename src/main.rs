use std::io::{self, Write};
use std::fs;
use std::process::Command;
use clap::{Parser, ValueEnum};

#[derive(Parser)]
#[command(name = "project-gen")]
#[command(about = "Projects Templates Generator")]
struct Cli {
    #[arg(short = 't', long = "type", value_enum)]
    project_type: Option<ProjectType>,

    #[arg(short = 'n', long = "name")]
    name: Option<String>,

    #[arg(short = 'a', long = "analyze")]
    analyze: Option<String>,

    #[arg(short = 'e', long = "tech")]
    tech: Option<String>,

    #[arg(short='d', long = "dir")]
    dir: Option<String>,
}

#[derive(ValueEnum, Clone)]
enum ProjectType {
    Flask,
    Rust,
    Frontend,
    Flutter,
    Java,
    Ios,
}

fn draw_title () {
    let title = "PROJECT TEMPLATES GENERATOR";
    let sub = "Happy Coding";
    let width = 100;

    println!("╭{:─^width$}╮", "", width = width);
    println!("│{:^width$}│", title, width = width);
    println!("├{:─^width$}┤", "", width = width);
    println!("│ {:<width$} │", sub, width = width - 2);
    println!("╰{:─^width$}╯", "", width = width);
    println!();
}

fn create_tree(name: &str, tree: &[(&str, bool)]) {
    for (path, is_dir) in tree {
        let full_path = format!("{}/{}", name, path);
        if *is_dir {
            if let Err(e) = fs::create_dir_all(&full_path) {
                eprintln!("Error creating dir {}: {}", full_path, e);
            }
        } else {
            if let Err(e) = fs::File::create(&full_path) {
                eprintln!("Error creating file {}: {}", full_path, e);
            }
        }
    }
}

fn create_flask(name: &str) {
    let tree = [
        ("templates/", true), ("static/", true),
        ("app.py", false), ("requirements.txt", false),
        ("templates/index.html", false), ("templates/style.css", false),
    ];
    create_tree(name, &tree);
    println!("Flask project created in {}/", name);
}

fn create_rust(name: &str) {
    let status = Command::new("cargo").arg("new").arg(name).status().expect("cargo not found");
    if status.success() { println!("Rust project created"); } else { eprintln!("cargo new failed"); }
}

fn create_frontend(name: &str) {
    fs::create_dir_all(name).ok();
    let tree = [("index.html", false), ("style.css", false), ("app.js", false)];
    create_tree(name, &tree);
    println!("Frontend project created in {}/", name);
}

fn create_flutter(name: &str) {
    let tree = [
        ("assets/", true), ("lib/src/", true), ("lib/widgets/", true),
        ("lib/main.dart", false), ("tests/", true), ("pubspec.yaml", false),
    ];
    create_tree(name, &tree);
    println!("Flutter project created in {}/", name);
}

fn create_java(name: &str) {
    let tree = [
        ("src/main/java/com/me/app/", true),
        ("src/main/java/com/me/app/Main.java", false),
        ("src/main/resources/", true), ("target/", true), ("pom.xml", false),
    ];
    create_tree(name, &tree);
    println!("Java project created in {}/", name);
}

fn create_ios(name: &str) {
    fs::create_dir_all(name).ok();
    let app_swift = format!("App/{}.swift", name);
    let tests_swift = format!("Tests/{}Tests.swift", name);
    let tree: &[(&str, bool)] = &[
        ("App/", true), (app_swift.as_str(), false),
        ("Views/", true), ("Views/ContentView.swift", false),
        ("Resources/Assets.xcassets/", true),
        ("Resources/Assets.xcassets/Contents.json", false),
        ("Resources/Info.plist", false),
        ("Tests/", true), (tests_swift.as_str(), false),
    ];
    create_tree(name, tree);
    println!("iOS project created in {}/", name);
}

fn main() {
    let cli = Cli::parse();
    if let (Some(project_type), Some(name)) = (cli.project_type.clone(), cli.name.clone()) {
        match project_type {
            ProjectType::Flask => create_flask(&name),
            ProjectType::Rust => create_rust(&name),
            ProjectType::Frontend => create_frontend(&name),
            ProjectType::Flutter => create_flutter(&name),
            ProjectType::Java => create_java(&name),
            ProjectType::Ios => create_ios(&name),
        }
        return;
    }
    if let (Some(dir), Some(tech)) = (cli.dir.clone(), cli.tech.clone()) {
        let required_files = match tech.trim().to_lowercase().as_str() {
            "flask" => vec!["app.py", "templates/", "static/", "requirements.txt"],
            "rust" => vec!["Cargo.toml", "src/", "src/main.rs"],
            "frontend" => vec!["index.html", "index.css", "app.js"],
            "flutter" => vec!["assets/", "lib/", "lib/src/", "lib/widgets/", "lib/main.dart", "tests/", "pubspec.yaml"],
            "java" => vec!["src/", "src/main/java/com/me/app/Main.java", "src/main/resources/", "target/", "pom.xml"],
            "ios" => vec!["App/", "Views/", "Views/ContentView.swift", "Resources/", "Resources/Info.plist", "Tests/"],
            _ => vec![]
        };
        if required_files.is_empty() {
            println!("An error occured in the analyze of the directory");
        }
        else {
            println!("Analyze in {}", dir);
            let mut missing_files = 0;
            for item in required_files {
                let full_path = format!("{}/{}", dir, item);
                if std::path::Path::new(&full_path).exists() {
                    println!("The item {} is here", item);
                }
                else {
                    println!("Missing: {}", item);
                    missing_files = missing_files+1;
                }
            }
            if missing_files == 0 {
                println!("Your directory seems to be good");
            }
            else {
                println!("The analyze reported {} missing files/directories", missing_files);
            }
        }
        return;
    }
    draw_title();
    let mut start = String::new();
    print!("1.Create a new project from a template\n2.Sart an analyze of an existing project(1 or 2): ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut start)
        .expect("An error occured");
    if start.trim() == "1" {
        println!("Available projects templates :\nFlask\nRust (cargo new)\nFrontend (html, css, js)\nFlutter\nJava\nIOS");
        let mut project = String::new();
        print!("Pick one: ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut project)
            .expect("Reading error retry later");
        if project.trim().to_lowercase() == "flask" {
            let mut filename = String::new();
            print!("Project file name : ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut filename).expect("Error in reading retry");
            create_flask(filename.trim());
        }
        if project.trim().to_lowercase() == "rust" {
            let mut filename = String::new();
            print!("Project file name : ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut filename).expect("Error in reading retry");
            create_rust(filename.trim());
        }
        if project.trim().to_lowercase() == "frontend" {
            let mut filename = String::new();
            print!("Project file name : ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut filename).expect("Error in reading retry");
            create_frontend(filename.trim());
        }
        if project.trim().to_lowercase() == "flutter" {
            let mut filename = String::new();
            print!("Project file name : ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut filename).expect("Error in reading retry");
            create_flask(filename.trim());
        }
        if project.trim().to_lowercase() == "java" {
            let mut filename = String::new();
            print!("Project file name : ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut filename).expect("Error in reading retry");
            create_java(filename.trim());
        }
        if project.trim().to_lowercase() == "ios" {
            let mut filename = String::new();
            print!("Project file name : ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut filename).expect("Error in reading retry");
            create_ios(filename.trim());
        }
    }
    else if start.trim() == "2" {
        let mut directory = String::new();
        print!("Which directory to analyze: ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut directory)
            .expect("An error occured");
        let dir = directory.trim();
        let mut tech = String::new();
        print!("Chose in supported technologies for scanning: Flask, Rust, Flutter, IOS, Java, Frontend: ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut tech)
            .expect("An error occured");
        let required_files = match tech.trim().to_lowercase().as_str() {
            "flask" => vec!["app.py", "templates/", "static/", "requirements.txt"],
            "rust" => vec!["Cargo.toml", "src/", "src/main.rs"],
            "frontend" => vec!["index.html", "index.css", "app.js"],
            "flutter" => vec!["assets/", "lib/", "lib/src/", "lib/widgets/", "lib/main.dart", "tests/", "pubspec.yaml"],
            "java" => vec!["src/", "src/main/java/com/me/app/Main.java", "src/main/resources/", "target/", "pom.xml"],
            "ios" => vec!["App/", "Views/", "Views/ContentView.swift", "Resources/", "Resources/Info.plist", "Tests/"],
            _ => vec![]
        };
        if required_files.is_empty() {
            println!("An error occured in the analyze of the directory");
        }
        else {
            println!("Analyze in {}", dir);
            let mut missing_files = 0;
            for item in required_files {
                let full_path = format!("{}/{}", dir, item);
                if std::path::Path::new(&full_path).exists() {
                    println!("The item {} is here", item);
                }
                else {
                    println!("Missing: {}", item);
                    missing_files = missing_files+1;
                }
            }
            if missing_files == 0 {
                println!("Your directory seems to be good");
            }
            else {
                println!("The analyze reported {} missing files/directories", missing_files);
            }
        }
    }
}
