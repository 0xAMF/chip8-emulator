# Chip8 Emulator

Welcome to the **Chip8 Emulator** project! This repository contains a Rust-based implementation of the Chip8 virtual machine, enabling you to run and interact with programs written for this classic platform.

## Overview

The Chip8 is a simple, interpreted programming language and virtual machine originally developed in the 1970s to create video games. This emulator replicates the Chip8 virtual machine, allowing users to enjoy and experiment with classic Chip8 programs.

## Features

- Fully functional Chip8 CPU and memory implementation.
- Accurate support for Chip8 opcodes.
- Display rendering powered by modern graphics libraries.
- Keyboard input mapping for the Chip8 keypad.
- Ability to load and execute Chip8 ROMs seamlessly.

## Prerequisites

To build and run the emulator, ensure you have the following installed:

- **Rust programming language**: Install it using [rustup](https://rustup.rs/):
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

- **Cargo**: The Rust package manager (included with Rust).

- **SDL2 library**: Required for graphics and input handling. Install SDL2 as per your operating system:

  - **Ubuntu/Debian**:
    ```bash
    sudo apt install libsdl2-dev
    ```

  - **MacOS**:
    ```bash
    brew install sdl2
    ```

  - **Windows**:
    Download the development libraries for SDL2 from [libsdl.org](https://www.libsdl.org/).

## Building the Project

1. Clone the repository:
   ```bash
   git clone https://github.com/0xAMF/chip8-emulator.git
   cd chip8-emulator
   ```

2. Build the project using Cargo:
   ```bash
   cargo build
   ```

3. The compiled executable will be located in the `target/release` directory.

## Usage

1. Run the emulator with a Chip8 ROM:
   ```bash
   cargo run -- path/to/ROM
   ```

2. Use the keyboard to interact with the Chip8 program. The key mapping corresponds to the following layout:
   ```
   1 2 3 4       ->   1 2 3 C
   Q W E R       ->   4 5 6 D
   A S D F       ->   7 8 9 E
   Z X C V       ->   A 0 B F
   ```
---
