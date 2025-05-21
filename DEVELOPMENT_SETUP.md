# RustQuote Service - Local Development Setup

This guide provides instructions for setting up a local development environment for the RustQuote Service project.

## 1. Prerequisites: Necessary Tools

Ensure you have the following tools installed on your system:

*   **Rust Toolchain:** This includes `rustc` (the Rust compiler) and `cargo` (the Rust package manager and build tool).
*   **Git:** For version control and cloning the repository.
*   **IDE/Text Editor:** A code editor with good Rust support. Visual Studio Code with the `rust-analyzer` extension is highly recommended.

## 2. Tool Installation

### 2.1. Rust Toolchain (rustc and cargo)

The recommended way to install Rust is by using `rustup`, the Rust toolchain installer.

*   **Linux/macOS:**
    Open your terminal and run:
    ```bash
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```
    Follow the on-screen instructions. After installation, make sure to configure your current shell:
    ```bash
    source $HOME/.cargo/env
    ```
    Or, open a new terminal window.

*   **Windows:**
    Download and run `rustup-init.exe` from [rustup.rs](https://rustup.rs/). Follow the installer prompts.

To verify the installation, open a new terminal and run:
```bash
rustc --version
cargo --version
```
You should see the installed versions of `rustc` and `cargo`.

### 2.2. Git

*   **Linux:**
    ```bash
    sudo apt-get update
    sudo apt-get install git  # Debian/Ubuntu
    # or
    sudo yum install git      # Fedora/CentOS
    ```
*   **macOS:**
    Git comes pre-installed with Xcode Command Line Tools. If not present, running `git` in the terminal will prompt you to install them. Alternatively, download an installer from the [official Git website](https://git-scm.com/download/mac).
*   **Windows:**
    Download and install Git for Windows from the [official Git website](https://git-scm.com/download/win).

### 2.3. IDE/Text Editor (VS Code Recommended)

*   Download and install [Visual Studio Code](https://code.visualstudio.com/) if you don't have it already.
*   **Recommended VS Code Extensions for Rust Development:**
    *   **`rust-analyzer`**: Provides language support (autocompletion, go to definition, linting, etc.). This is the official Rust language server.
    *   **`crates`**: Helps manage `Cargo.toml` dependencies.
    *   **`Better TOML`**: Syntax highlighting and validation for TOML files (like `Cargo.toml`).
    *   **`CodeLLDB`**: For debugging Rust code (alternative to `ms-vscode.cpptools` for Rust).

    You can install these extensions from the Extensions view (Ctrl+Shift+X or Cmd+Shift+X) in VS Code by searching for their names.

## 3. Cloning the Repository

1.  Open your terminal or command prompt.
2.  Navigate to the directory where you want to clone the project.
3.  Run the following command (replace `<repository_url>` with the actual URL of the RustQuote Service repository):
    ```bash
    git clone <repository_url>
    cd rustquote_service  # Or the actual repository directory name
    ```

## 4. Building the Project

Once you have cloned the repository and navigated into its directory, you can build the project using Cargo:

```bash
cargo build
```
This command compiles your project. If you want to build a release version (optimized for performance), use:
```bash
cargo build --release
```

## 5. Running the Project

To compile and run the project in development mode:

```bash
cargo run
```
This command will build the project if necessary and then execute the main binary.

If you have built a release version, you can run the executable directly from the `target/release` directory or use:
```bash
cargo run --release
```

## 6. Running Tests

To run the automated tests for the project:

```bash
cargo test
```
Currently, there might not be extensive tests, but this is the standard command to execute them. As the project develops, tests will be added to ensure code quality and correctness.

## 7. Common Troubleshooting & Documentation

*   **Rustup Issues:** If you encounter issues with `rustup` or managing toolchains, refer to the [Rustup documentation](https://rust-lang.github.io/rustup/).
*   **Cargo Issues:** For problems related to dependencies, building, or running the project with Cargo, consult the [Cargo Book](https://doc.rust-lang.org/cargo/).
*   **General Rust Questions:** The official [Rust Programming Language Book ("The Book")](https://doc.rust-lang.org/book/) is an excellent resource for learning Rust and understanding its concepts.
*   **`rust-analyzer` Issues:** If `rust-analyzer` is not working correctly in VS Code:
    *   Ensure it's installed and enabled.
    *   Check the `rust-analyzer` output panel in VS Code for error messages.
    *   Make sure your Rust installation is up to date (`rustup update`).
*   **Build Failures:**
    *   Carefully read the compiler error messages. Rust's compiler is known for its helpful error diagnostics.
    *   Ensure all dependencies specified in `Cargo.toml` are compatible and correctly fetched (`cargo clean` and `cargo build` can sometimes resolve issues).

## 8. Further Reading

*   [The Rust Programming Language Book](https://doc.rust-lang.org/book/)
*   [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
*   [Cargo Book](https://doc.rust-lang.org/cargo/)
*   [Rustup Documentation](https://rust-lang.github.io/rustup/)

---

This document will be updated as the project evolves.