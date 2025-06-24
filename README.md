# Rusty-8: A Chip-8 Emulator in Rust and WebAssembly

To be precise, Chip-8 is an interpreter, not an emulator. The goal of Chip-8 was to provide a universal language—similar to the Java JVM or the .NET CLR—for developing games. As far as I know, no Chip-8 processor has ever actually been built. For more information, see the [Wikipedia article](https://en.wikipedia.org/wiki/CHIP-8).

I came across Chip-8 while reading *Rust in Action: Systems Programming Concepts and Techniques*. The book briefly touches on Chip-8 in the context of CPU emulation. I therefore just went with the name "Chip-8 Emulator."

As a guide for the implementation, I used [Cowgod's technical reference](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM).

---

## Live Demo

**[> Try it out here! <](https://zui0per.github.io/Chip8/)**

---

## How to Play

The original Chip-8 systems used a 16-key hexadecimal keypad. This emulator maps those keys to a modern keyboard as follows:

| Original Keypad | Your Keyboard |
| :-------------: | :-----------: |
|    `1 2 3 C`    |   `1 2 3 4`   |
|    `4 5 6 D`    |   `Q W E R`   |
|    `7 8 9 E`    |   `A S D F`   |
|    `A 0 B F`    |   `Z X C V`   |

Use the on-screen controls to select and load different ROMs.

---

## Local Development and Building from Source

Want to run the project locally? Here’s how.
*Note: This process has not been tested.*

### Prerequisites

* **Rust:** Install from [rust-lang.org](https://www.rust-lang.org/tools/install).
* **`wasm-pack`:** A tool for building Rust-generated WebAssembly.

  ```bash
  cargo install wasm-pack
  ```
* **Node.js and npm:** Required for running the frontend development server. Install from [nodejs.org](https://nodejs.org/).

### Steps to Build

1. **Clone the repository:**

   ```bash
   git clone https://github.com/Zui0per/chip-8-emulator.git 
   ```
2. **Install dependencies:**

   ```bash
   cd web 
   npm ci
   ```
3. **Build the WebAssembly module:**

   This command compiles the Rust code into Wasm and generates the necessary JavaScript bindings, placing them in the `web/wasm` directory.

   ```bash
   npm run build-wasm
   ```
4. **Run the frontend:**

   ```bash
   npm run start
   ```

   The console will display which port the application is available on.

---

### Running Tests

To run the full suite of unit tests for the Chip-8 core logic:

```bash
npm run test
# or via Cargo
cd ..
cargo test 
```