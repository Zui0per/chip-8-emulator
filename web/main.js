// AI generated 
import init, { Emulator } from './chip_8_emulator.js';

// --- Global State ---
let emulator;
let animationFrameId = null;

// The list of ROMs you provided, ready for the dropdown.
/*
const ROMS = [
  "octajam_title", "red_october", "puzzle_15", "blinky", "blitz", "brix", "connect", 
  "guess", "hidden", "invaders", "kaleid", "maze", "merlin", "missile", 
  "pong", "pong2", "puzzle", "syzygy", "tank", "tetris", "tictac", "ufo", 
  "vbrix", "vers", "wipeoff"
];*/

const ROMS = [
  "octajam_title", "puzzle_15", 
  "guess", "invaders",
  "pong", "pong2", "tank",
];

const keyMap = {
  '1': 0x1,
  '2': 0x2,
  '3': 0x3,
  '4': 0xC,
  'q': 0x4,
  'w': 0x5,
  'e': 0x6,
  'r': 0xD,
  'a': 0x7,
  's': 0x8,
  'd': 0x9,
  'f': 0xE,
  'y': 0xA, // German keyboard (Y instead of Z)
  'x': 0x0,
  'c': 0xB,
  'v': 0xF
}; 

let currentRom = ROMS[0];

window.addEventListener('keydown', (e) => {
const chip8Key = keyMap[e.key.toLowerCase()];
if (chip8Key !== undefined) {
  emulator.set_key(chip8Key, true); // or whatever your key press handler is
}
});

window.addEventListener('keyup', (e) => {
const chip8Key = keyMap[e.key.toLowerCase()];
if (chip8Key !== undefined) {
  emulator.set_key(chip8Key, false);
}
});



// --- Main Application ---
async function run() {
  const module = await init();

  const beeper = new Beeper();

  // --- DOM Element References ---
  const romSelect = document.getElementById('rom-select');
  const canvas = document.getElementById('screen');
  const registersView = document.getElementById('registers-view');
  const instructionList = document.getElementById('instruction-list');
  const ctx = canvas.getContext('2d');
  
  const width = 64; // Hardcoded since we know the CHIP-8 size
  const height = 32;

  canvas.width = width;
  canvas.height = height;

  // --- Renderer Setup (using the fast putImageData method) ---
  const imageData = ctx.createImageData(width, height);
  const canvasPixelData = imageData.data;
  const onColor = [255, 255, 255, 255]; // White
  const offColor = [0, 0, 0, 255];     // Black

  function render() {
    const display_ptr = emulator.get_display_ptr();
    const chip8DisplayData = new Uint8Array(module.memory.buffer, display_ptr, width * height);

    for (let i = 0; i < chip8DisplayData.length; i++) {
        const canvasIdx = i * 4;
        const color = chip8DisplayData[i] ? onColor : offColor;
        canvasPixelData.set(color, canvasIdx);
    }
    ctx.putImageData(imageData, 0, 0);
  }

  // --- Debug UI Update Functions ---
  let last10Instructions = [];
  function updateDebugUI(newOpcodes) {
      // Update instruction list
      last10Instructions.unshift(...newOpcodes);
      if (last10Instructions.length > 10) {
        last10Instructions.length = 10;
      }
      instructionList.innerHTML = last10Instructions
        .map(op => `<li>0x${op.toString(16).toUpperCase().padStart(4, '0')}</li>`)
        .join('');

      // Update registers view
      const snapshot = emulator.get_register_snapshot();
      registersView.innerHTML = `
        <div><span class="reg-name">V0:</span> <span>0x${snapshot.V0.toString(16).toUpperCase().padStart(2, '0')}</span></div>
        <div><span class="reg-name">V1:</span> <span>0x${snapshot.V1.toString(16).toUpperCase().padStart(2, '0')}</span></div>
        <div><span class="reg-name">V2:</span> <span>0x${snapshot.V2.toString(16).toUpperCase().padStart(2, '0')}</span></div>
        <div><span class="reg-name">V3:</span> <span>0x${snapshot.V3.toString(16).toUpperCase().padStart(2, '0')}</span></div>
        <div><span class="reg-name">V4:</span> <span>0x${snapshot.V4.toString(16).toUpperCase().padStart(2, '0')}</span></div>
        <div><span class="reg-name">V5:</span> <span>0x${snapshot.V5.toString(16).toUpperCase().padStart(2, '0')}</span></div>
        <div><span class="reg-name">V6:</span> <span>0x${snapshot.V6.toString(16).toUpperCase().padStart(2, '0')}</span></div>
        <div><span class="reg-name">V7:</span> <span>0x${snapshot.V7.toString(16).toUpperCase().padStart(2, '0')}</span></div>
        <div><span class="reg-name">V8:</span> <span>0x${snapshot.V8.toString(16).toUpperCase().padStart(2, '0')}</span></div>
        <div><span class="reg-name">V9:</span> <span>0x${snapshot.V9.toString(16).toUpperCase().padStart(2, '0')}</span></div>
        <div><span class="reg-name">VA:</span> <span>0x${snapshot.VA.toString(16).toUpperCase().padStart(2, '0')}</span></div>
        <div><span class="reg-name">VB:</span> <span>0x${snapshot.VB.toString(16).toUpperCase().padStart(2, '0')}</span></div>
        <div><span class="reg-name">VC:</span> <span>0x${snapshot.VC.toString(16).toUpperCase().padStart(2, '0')}</span></div>
        <div><span class="reg-name">VD:</span> <span>0x${snapshot.VD.toString(16).toUpperCase().padStart(2, '0')}</span></div>
        <div><span class="reg-name">VE:</span> <span>0x${snapshot.VE.toString(16).toUpperCase().padStart(2, '0')}</span></div>
        <div><span class="reg-name">VF:</span> <span>0x${snapshot.VF.toString(16).toUpperCase().padStart(2, '0')}</span></div>
        <div><span class="reg-name">I:</span>  <span>0x${snapshot.I.toString(16).toUpperCase().padStart(4, '0')}</span></div>
        <div><span class="reg-name">PC:</span> <span>0x${snapshot.programm_counter.toString(16).toUpperCase().padStart(4, '0')}</span></div>
        <div><span class="reg-name">DT:</span> <span>${snapshot.delay_timer}</span></div>
        <div><span class="reg-name">ST:</span> <span>${snapshot.sound_timer}</span></div>
      `;
  }

  // --- Game Loop ---
  let lastTime = 0;
  let lastDebugUpdateTime = 0;
  const TARGET_CPS = 700; // Cycles per second

  function game_loop(currentTime) {
    animationFrameId = requestAnimationFrame(game_loop);
    let deltaTime = currentTime - lastTime;
    
    if (deltaTime > 0) {
      const cyclesToRun = deltaTime * (TARGET_CPS / 1000);
      const opcodesThisFrame = [];

      emulator.update_timers(deltaTime);

      for (let i = 0; i < cyclesToRun; i++) {
        const opcode = emulator.execute_instruction();
        opcodesThisFrame.push(opcode);
      }
      
      render();
      updateDebugUI(opcodesThisFrame);
      lastDebugUpdateTime = currentTime;

      if (emulator.is_sound_active())
      {
        beeper.start()
      }
      else {
        beeper.stop();
      }
    }

    lastTime = currentTime;
  }

  // --- Emulation Control ---
  function startEmulator(romName) {

    if (animationFrameId) {
      cancelAnimationFrame(animationFrameId);
      animationFrameId = null;
    }
    
    // Create a fresh emulator instance for the new ROM
    emulator = new Emulator();
    emulator.load_rom(romName);

    // Reset UI elements
    last10Instructions = [];
    instructionList.innerHTML = '';
    
    // Start the loop
    lastTime = 0;
    lastDebugUpdateTime = 0;
    game_loop(0);
  }

  // --- Initial Setup ---
  ROMS.forEach(rom => {
    const option = document.createElement('option');
    option.value = rom;
    // Capitalize first letter for display
    option.textContent = rom.charAt(0).toUpperCase() + rom.slice(1).replace('_', ' ');
    romSelect.appendChild(option);
  });

  romSelect.addEventListener('change', (event) => {
    currentRom = event.target.value;
    startEmulator(event.target.value);
  });

  document.addEventListener('visibilitychange', () => {
    if (document.hidden) {
      if (animationFrameId !== null) {
        cancelAnimationFrame(animationFrameId);
        animationFrameId = null;
        beeper.stop();
      }
      emulator = null;
    } else {
      if (!animationFrameId) {
        startEmulator(currentRom);
      }
    }
  });
  // Load the first ROM by default
  startEmulator(currentRom);
}

run();

class Beeper {
  constructor() {
    // Create an AudioContext. This must be done after a user interaction (like a click).
    // We will initialize it later inside an event listener.
    this.audioCtx = null;
    this.oscillator = null;
    this.gainNode = null;
    this.isPlaying = false;
  }

  _init() {
    // This private method sets up the audio graph
    if (this.audioCtx) return; // Already initialized

    this.audioCtx = new (window.AudioContext || window.webkitAudioContext)();
    
    // Create an oscillator for the tone
    this.oscillator = this.audioCtx.createOscillator();
    this.oscillator.type = 'square'; // 'sine', 'square', 'sawtooth', 'triangle'
    this.oscillator.frequency.setValueAtTime(440, this.audioCtx.currentTime); // 440 Hz is A4

    // Create a gain node to control the volume
    this.gainNode = this.audioCtx.createGain();
    this.gainNode.gain.setValueAtTime(0, this.audioCtx.currentTime); // Start with volume 0

    // Connect the nodes: Oscillator -> Gain -> Destination (speakers)
    this.oscillator.connect(this.gainNode);
    this.gainNode.connect(this.audioCtx.destination);
    
    // Start the oscillator. It will run silently until we turn up the gain.
    this.oscillator.start();
  }

  start() {
    // Browsers require a user gesture (like a click) to start audio.
    // _init() will create the context if it doesn't exist.
    this._init();
    
    if (!this.isPlaying && this.gainNode) {
      // Ramp up the volume to avoid a "click" sound
      this.gainNode.gain.setTargetAtTime(0.005, this.audioCtx.currentTime, 0.01);
      this.isPlaying = true;
    }
  }

  stop() {
    if (this.isPlaying && this.gainNode) {
      // Ramp down the volume
      this.gainNode.gain.setTargetAtTime(0, this.audioCtx.currentTime, 0.01);
      this.isPlaying = false;
    }
  }
}