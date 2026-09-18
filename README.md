# Project Generator

A CLI tool to quickly scaffold project templates for Flask, Rust (Cargo), Frontend (HTML/CSS/JS), Flutter, Java, and iOS.

---

## Prerequisites — Install Rust & Cargo

### Linux / macOS

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Then restart your terminal or run:

```bash
source $HOME/.cargo/env
```

### Windows

Download and run the installer from the official page:
**https://www.rust-lang.org/tools/install**

Follow the on-screen instructions. This installs both `rustup`, `rustc`, and `cargo`.

Verify the installation:

```bash
rustc --version
cargo --version
```

---

## Installation

### 1. Clone the repository

```bash
git clone https://github.com/DankDown10256/rust_project_templates_generator/
```

### 2. Enter the directory

```bash
cd rust_project_templates_generator
```

### 3. Build the project

```bash
cargo build --release
```

### 4. Add to PATH (optional)

#### Linux / macOS

```bash
sudo cp target/release/project_generator /usr/local/bin/project-gen
```

#### Windows (PowerShell — run as Administrator)

```powershell
Copy-Item "target\release\project_generator.exe" "C:\Windows\System32\project-gen.exe"
```

Or add `target\release\` to your `PATH` environment variable and rename the binary to `project-gen.exe`.

---

## Usage

### Create a project

```bash
project-gen -t <type> -n <name>
```

#### Available types

| Type | Description |
|------|-------------|
| `flask` | Python Flask app (`app.py`, `templates/`, `static/`, `requirements.txt`) |
| `rust` | Rust Cargo project (`cargo new`) |
| `frontend` | HTML/CSS/JS (`index.html`, `style.css`, `app.js`) |
| `flutter` | Flutter/Dart app (`lib/`, `pubspec.yaml`, `tests/`) |
| `java` | Java Maven project (`pom.xml`, `src/main/java/`) |
| `ios` | Swift/Xcode project (`App/`, `Views/`, `Resources/`, `Tests/`) |

#### Examples

```bash
project-gen -t flask -n my-api
project-gen -t rust -n my-lib
project-gen -t frontend -n landing-page
project-gen -t flutter -n my-app
project-gen -t java -n my-service
project-gen -t ios -n my-ios-app
```

### Analyze an existing project

Check if a directory contains the expected files for a given technology:

```bash
project-gen -d <directory> -e <tech>
```

#### Example

```bash
project-gen -d ./my-project -e flask
```

This will report which expected files are present or missing.

---

## Contribution/Feedbacks

All feedbacks are welcome. If you want to help to upgrade this project you can contribute by checking the rules here [CONTRIBUTING.md](CONTRIBUTING.md).
