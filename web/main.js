// AI generated 
import init, { Emulator } from './chip_8_emulator.js';

async function run() {
  const module = await init();
  const emulator = new Emulator();
  const canvas = document.getElementById('screen');
  const ctx = canvas.getContext('2d');
  
  const width = emulator.get_display_width();
  const height = emulator.get_display_height();

  canvas.width = width;
  canvas.height = height;
  const scale = 5;
  canvas.style.width = canvas.width * scale + 'px';
  canvas.style.height = canvas.height * scale + 'px';
  ctx.imageSmoothingEnabled = false;

  const keyMap = { /* ... your keymap ... */ };
  window.addEventListener('keydown', e => { /* ... */ });
  window.addEventListener('keyup', e => { /* ... */ });

  // ====================================================================
  // --- THE NEW, FAST RENDER SETUP ---
  // ====================================================================
  
  // 1. Create the ImageData object ONCE. We will reuse it every frame.
  const imageData = ctx.createImageData(width, height);
  // Get a direct reference to its pixel buffer.
  const canvasPixelData = imageData.data;

  // Define our ON and OFF colors as RGBA arrays.
  const onColor = [255, 255, 255, 255]; // White
  const offColor = [0, 0, 0, 255];     // Black

  function render() {
    // Get the latest display state from WASM.
    const display_ptr = emulator.get_display_ptr();
    const chip8DisplayData = new Uint8Array(module.memory.buffer, display_ptr, width * height);

    // 2. Loop through our CHIP-8 display data and update our canvas pixel buffer.
    for (let i = 0; i < chip8DisplayData.length; i++) {
        // Find the starting index in the RGBA buffer.
        const canvasIdx = i * 4;
        const color = chip8DisplayData[i] ? onColor : offColor;

        canvasPixelData[canvasIdx]     = color[0]; // R
        canvasPixelData[canvasIdx + 1] = color[1]; // G
        canvasPixelData[canvasIdx + 2] = color[2]; // B
        canvasPixelData[canvasIdx + 3] = color[3]; // A
    }

    // 3. Make ONE call to put the entire updated buffer onto the canvas.
    ctx.putImageData(imageData, 0, 0);
  }

  // --- The Game Loop (remains the same) ---
  let lastTime = 0;
  const TARGET_CPS = 700; 

  function game_loop(currentTime) {
    requestAnimationFrame(game_loop);
    const deltaTime = currentTime - lastTime;
    
    if (deltaTime > 0) {
      const cyclesToRun = deltaTime * (TARGET_CPS / 1000);
      emulator.update_timers(deltaTime);
      for (let i = 0; i < cyclesToRun; i++) {
          emulator.execute_instruction();
      }
      render(); // Call our new, fast render function
      if (emulator.is_sound_active()) { /* ... */ }
    }

    lastTime = currentTime;
  }

  requestAnimationFrame(game_loop);
}

run();