# 🎸 Fretboard Training

A random note generator to help guitarists learn and memorize their guitar fretboard.

## 📋 Description

This application displays random notes that you need to find on your guitar fretboard. It's a simple and effective tool to improve your fretboard knowledge.

## ✨ Features

- Random note generation
- Two modes:
  - **Natural notes only mode**: only natural notes (C, D, E, F, G, A, B)
  - **Accidentals mode**: natural notes + sharps and flats (C#, D#, F#, G#, A#, Db, Eb, Gb, Ab, Bb)
- Two notation formats:
  - **English format**: C, D, E, F, G, A, B (with sharps/flats)
  - **Classical format**: Do, Ré, Mi, Fa, Sol, La, Si (with sharps/flats)
- Simple and intuitive graphical interface

## 🛠️ Prerequisites

To compile and run this project, you need to have installed:

- **Rust** (version 1.70 or higher recommended)
- **Cargo** (usually included with Rust)

### Installing Rust

If Rust is not yet installed on your system:

#### Windows
1. Download the installer from [rustup.rs](https://rustup.rs/)
2. Run `rustup-init.exe`
3. Follow the on-screen instructions

#### Linux / macOS
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

After installation, restart your terminal or run:
```bash
source $HOME/.cargo/env
```

Verify the installation:
```bash
rustc --version
cargo --version
```

## 🔨 Compilation

This section provides platform-specific instructions for compiling the project.

### Windows

On Windows, the project compiles natively using the MSVC toolchain (default).

1. **Ensure Rust is installed** (see Prerequisites section above)

2. **Build the project**:
   ```bash
   cargo build --release
   ```

3. **The executable** will be located at:
   ```
   target/release/note-generator.exe
   ```

**Note**: The project uses `winres` as a build dependency to set Windows-specific resources (icon, version info, etc.). This is automatically handled during compilation.

### macOS

On macOS, the project can be compiled for both Intel (x86_64) and Apple Silicon (ARM64) architectures.

1. **Ensure Rust is installed** (see Prerequisites section above)

2. **Install Xcode Command Line Tools** (if not already installed):
   ```bash
   xcode-select --install
   ```

3. **Add the appropriate target** (if needed):
   - For Apple Silicon (M1/M2/M3):
     ```bash
     rustup target add aarch64-apple-darwin
     ```
   - For Intel Macs:
     ```bash
     rustup target add x86_64-apple-darwin
     ```

4. **Build the project**:
   - For your current architecture:
     ```bash
     cargo build --release
     ```
   - For a specific architecture (e.g., Apple Silicon):
     ```bash
     cargo build --release --target aarch64-apple-darwin
     ```

5. **The executable** will be located at:
   ```
   target/release/note-generator
   ```
   or for a specific target:
   ```
   target/aarch64-apple-darwin/release/note-generator
   ```

## 📦 Project Installation

1. Clone or download this project
2. Open a terminal in the project folder
3. Dependencies will be automatically downloaded during the first compilation

## 🚀 Building and Running

### Building

To build the project in debug mode (development):
```bash
cargo build
```

To build in release mode (optimized):
```bash
cargo build --release
```

The executable will be created in:
- Debug mode: `target/debug/note-generator.exe` (Windows) or `target/debug/note-generator` (Linux/macOS)
- Release mode: `target/release/note-generator.exe` (Windows) or `target/release/note-generator` (Linux/macOS)

### Running

To build and run directly:
```bash
cargo run
```

To run the optimized version:
```bash
cargo run --release
```

## 🎮 Usage

1. Launch the application
2. A random note is automatically displayed
3. Check/uncheck "Mode with accidentals" to change the type of notes generated
4. Check/uncheck "Classical format (Do Ré Mi...)" to switch between English and classical notation
5. Click "🎲 New Note" to generate a new random note
6. Find the displayed note on your guitar fretboard!

## 🧪 Tests

To run unit tests:
```bash
cargo test
```

## 📁 Project Structure

```
note-generator/
├── Cargo.toml          # Project configuration and dependencies
├── README.md           # This file
└── src/
    ├── main.rs         # Entry point and graphical interface
    └── note_generator.rs # Note generation module
```

## 🔧 Dependencies

- **eframe**: Framework for creating native applications with egui
- **egui**: Immediate mode graphical user interface library
- **rand**: Library for random number generation

## 📝 Notes

- This application is designed to be simple and efficient
- Notes are generated randomly and fairly
- The interface is optimized for quick use during practice

## 🤝 Contributing

Contributions are welcome! Feel free to propose improvements.

## 📄 License

This project is free to use for learning and musical training.

---

Happy practicing! 🎸🎵
