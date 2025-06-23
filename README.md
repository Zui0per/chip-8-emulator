# Rusty-8: A Chip-8 Emulator in Rust and WebAssembly

To be correct Chip-8 is an interpreter and not an emulator. The goal of Chip-8 was to provide a universal language, similair to the Java JVM or the .NET CLR, to develop games. As far as I know,
no Chip-8 processor has actually been built. For more information see the [wikipedia article](https://en.wikipedia.org/wiki/CHIP-8).

I came across Chip-8 when reading "Rust in Action Systems programming concepts and techniques". The book touches a bit on Chip-8 in the context of CPU emulaton. I therfore just went with the name Chip-8 Emulator.

TODO
![Gameplay GIF of Rusty-8 playing PONG](https://path-to-your/gameplay.gif)

---

## Live Demo

**[> Try it out here! <](https://your-username.github.io/your-repo-name/)**

*(Host your project on GitHub Pages, Vercel, or Netlify and link to it here.)*

## Features

*   **Rust Core:** Logic is written in safe, high-performance Rust.
*   **WebAssembly Target:** Runs at near-native speed directly in your web browser.
*   **Full Instruction Set:** Implements all 35 standard Chip-8 opcodes.
*   **Sound and Timers:** Accurate 60Hz timers and beeping sound effects.
*   **ROM Library:** Comes pre-loaded with several classic Chip-8 games and demos.
*   **Debug View:** (Optional, but a great feature if you have it) A real-time view of registers, timers, and the program counter.

## How to Play

The original Chip-8 systems used a 16-key hexadecimal keypad. This emulator maps those keys to a modern keyboard as follows:

| Original Keypad | Your Keyboard |
| :-------------: | :-----------: |
| `1 2 3 C`       | `1 2 3 4`     |
| `4 5 6 D`       | `Q W E R`     |
| `7 8 9 E`       | `A S D F`     |
| `A 0 B F`       | `Z X C V`     |

Use the on-screen controls to select and load different ROMs.

## Local Development and Building from Source

Want to run the project locally or contribute? Here’s how.

### Prerequisites

*   **Rust:** Install from [rust-lang.org](https://www.rust-lang.org/tools/install).
*   **`wasm-pack`:** The tool for building Rust-generated WebAssembly.
    ```bash
    cargo install wasm-pack
    ```
*   **Node.js and npm:** For running the frontend development server. Install from [nodejs.org](https://nodejs.org/).

### Steps to Build

1.  **Clone the repository:**
    ```bash
    git clone https://github.com/your-username/your-repo-name.git
    cd your-repo-name
    ```

2.  **Build the WebAssembly module:**
    This command compiles the Rust code into Wasm and generates the necessary JavaScript bindings, placing them in the `frontend/src/wasm` directory (or wherever your output path is).
    ```bash
    # From the project root
    wasm-pack build ./chip8_core --target web --out-dir ./frontend/src/wasm
    ```
    *(Adjust the paths to match your project structure. You can also add this as a `cargo` alias or `npm` script as we discussed.)*

3.  **Run the frontend:**
    ```bash
    cd frontend
    npm install
    npm run dev
    ```
    The application should now be running on `http://localhost:5173` (or similar).

### Running Tests

To run the full suite of unit tests for the Chip-8 core logic:
```bash
# From the chip8_core directory
cargo test