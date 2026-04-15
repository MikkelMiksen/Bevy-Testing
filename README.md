# Bevy-Testing

Figuring out the basics of making a game with Rust as the primary language, with the purpose of one day building our own game engine in the language.

## Overview

This repository is a sandbox for learning [Bevy](https://bevyengine.org/), a data-driven game engine built in Rust. It explores core ECS (Entity Component System) concepts, plugin architecture, and system orchestration.

The current implementation includes:
-   A basic ECS setup with `Person` and `Name` components.
-   A custom `HelloPlugin` to handle system registration.
-   System chaining for orderly execution of startup and update logic.
-   Integration with `bevy_egui` and `egui` (as indicated by dependencies).

## Requirements

-   [Rust](https://www.rust-lang.org/tools/install) (Edition 2024 recommended)
-   [Cargo](https://doc.rust-lang.org/cargo/) (Rust's package manager)
-   OS-specific Bevy dependencies (e.g., Alsa/Vulkan on Linux, though this project seems to be on Windows)

## Setup & Run

### 1. Clone the repository
```powershell
git clone https://github.com/your-username/Bevy-Testing.git
cd Bevy-Testing
```

### 2. Run the application
To run the project in debug mode:
```powershell
cargo run
```

To run in release mode (optimized):
```powershell
cargo run --release
```

## Scripts

Currently, the project uses standard Cargo commands:
-   `cargo run`: Compile and run the project.
-   `cargo build`: Build the project without running.
-   `cargo check`: Fast compilation check.
-   `cargo fmt`: Format the code (requires `rustfmt`).
-   `cargo clippy`: Run linting (requires `clippy`).

*Note: There are no custom scripts (e.g., `.sh` or `.ps1`) in the root directory.*

## Environment Variables

-   `RUST_LOG`: Set to `info` or `debug` to see Bevy logs (e.g., `$env:RUST_LOG="bevy=info"; cargo run` in PowerShell).
-   `WGPU_BACKEND`: Can be used to force a specific graphics backend (vulkan, dx12, metal, etc.).

## Tests

Currently, no automated tests are implemented.

-   **TODO**: Implement unit tests for systems and plugins.
-   Run tests (when added): `cargo test`

## Project Structure

```text
.
├── Cargo.toml         # Manifest with Bevy and egui dependencies
├── LICENSE            # License information
├── README.md          # Project documentation
└── src
    └── main.rs        # Main entry point and ECS logic
```

## License

This project is licensed under the terms found in the [LICENSE](LICENSE) file.