/* tslint:disable */
/* eslint-disable */
export class Emulator {
  free(): void;
  constructor();
  set_key(key: number, is_pressed: boolean): void;
  get_framebuffer_ptr(): number;
  get_framebuffer_len(): number;
  execute_instruction(): void;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_emulator_free: (a: number, b: number) => void;
  readonly emulator_new: () => number;
  readonly emulator_set_key: (a: number, b: number, c: number) => void;
  readonly emulator_get_framebuffer_ptr: (a: number) => number;
  readonly emulator_get_framebuffer_len: (a: number) => number;
  readonly emulator_execute_instruction: (a: number) => void;
  readonly __wbindgen_export_0: WebAssembly.Table;
  readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
*
* @returns {InitOutput}
*/
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
