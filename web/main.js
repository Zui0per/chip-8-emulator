// AI generated 
import init, { Emulator } from './chip_8_emulator.js';

async function run() {
  const module = await init();

  const emulator = new Emulator();

  const canvas = document.getElementById('screen');
  const ctx = canvas.getContext('2d');
  canvas.width = 64;
  canvas.height = 32;

  // Scale up for visibility
  const scale = 10;
  canvas.style.width = canvas.width * scale + 'px';
  canvas.style.height = canvas.height * scale + 'px';

  // Keyboard map (your CHIP-8 keys to physical keys)
  const keyMap = {
    '1': 0x1, '2': 0x2, '3': 0x3, '4': 0xC,
    'q': 0x4, 'w': 0x5, 'e': 0x6, 'r': 0xD,
    'a': 0x7, 's': 0x8, 'd': 0x9, 'f': 0xE,
    'z': 0xA, 'x': 0x0, 'c': 0xB, 'v': 0xF,
  };

  window.addEventListener('keydown', e => {
    const key = keyMap[e.key];
    if (key !== undefined) {
      emulator.set_key(key, true);
      e.preventDefault();
    }
  });

  window.addEventListener('keyup', e => {
    const key = keyMap[e.key];
    if (key !== undefined) {
      emulator.set_key(key, false);
      e.preventDefault();
    }
  });

  function render() {
    const fbPtr = emulator.get_framebuffer_ptr();
    const fbLen = emulator.get_framebuffer_len();
    // The framebuffer is a pointer to wasm memory boolean array
    const fb = new Uint8Array(module.memory.buffer, fbPtr, fbLen);

    ctx.fillStyle = 'black';
    ctx.fillRect(0, 0, canvas.width, canvas.height);

    ctx.fillStyle = 'white';
    for (let i = 0; i < fbLen; i++) {
      if (fb[i]) {
        const x = i % canvas.width;
        const y = Math.floor(i / canvas.width);
        ctx.fillRect(x, y, 1, 1);
      }
    }
  }

  function loop() {
    emulator.execute_instruction();
    render();
    requestAnimationFrame(loop);
  }

  loop();
}

run();
