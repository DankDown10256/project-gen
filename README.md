# Project Generator

A CLI tool to quickly scaffold project templates for Flask, Rust (Cargo), and Frontend (HTML/CSS/JS).

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

### 4. Run the tool

```bash
cargo run
```

---

## Add to PATH as `project-gen`

To run the tool from anywhere with the command `project-gen`:

### Linux / macOS

```bash
sudo cp target/release/project_generator /usr/local/bin/project-gen
```

You can now run it from any directory:

```bash
project-gen
```

### Windows (PowerShell — run as Administrator)

```powershell
Copy-Item "target\release\project_generator.exe" "C:\Windows\System32\project-gen.exe"
```

Or, alternatively, add the `target\release\` folder to your `PATH` environment variable:

1. Open **System Properties** → **Environment Variables**
2. Under **User variables**, select `Path` and click **Edit**
3. Click **New** and add the full path to `target\release\`
4. Click **OK** and restart your terminal

Then create an alias or rename the binary to `project-gen.exe`.

---

## Usage

When launched, the tool displays available templates:

```
Available projects templates :
Flask
Rust (cargo)
frontend (html, css, js)
```

Pick a template, enter a project name, and the files are generated instantly.

# Contribution/Feedbacks

All feedbacks are welcome. If you want to help to upgrade this project you can contribute by checking the rules here [CONTRIBUTING.md](CONTRIBUTING.md).
