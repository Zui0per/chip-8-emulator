/* tslint:disable */
/* eslint-disable */
export class Emulator {
  free(): void;
  constructor();
  set_key(key: number, is_pressed: boolean): void;
  get_display_ptr(): number;
  get_display_width(): number;
  get_display_height(): number;
  execute_instruction(): number;
  update_timers(elapsed_ms: number): void;
  is_sound_active(): boolean;
  load_rom(name: string): void;
  get_register_snapshot(): RegistersSnapshot;
}
export class RegistersSnapshot {
  private constructor();
  free(): void;
  V0: number;
  V1: number;
  V2: number;
  V3: number;
  V4: number;
  V5: number;
  V6: number;
  V7: number;
  V8: number;
  V9: number;
  VA: number;
  VB: number;
  VC: number;
  VD: number;
  VE: number;
  VF: number;
  I: number;
  delay_timer: number;
  sound_timer: number;
  programm_counter: number;
  stack_pointer: number;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_emulator_free: (a: number, b: number) => void;
  readonly __wbg_registerssnapshot_free: (a: number, b: number) => void;
  readonly __wbg_get_registerssnapshot_V0: (a: number) => number;
  readonly __wbg_set_registerssnapshot_V0: (a: number, b: number) => void;
  readonly __wbg_get_registerssnapshot_V1: (a: number) => number;
  readonly __wbg_set_registerssnapshot_V1: (a: number, b: number) => void;
  readonly __wbg_get_registerssnapshot_V2: (a: number) => number;
  readonly __wbg_set_registerssnapshot_V2: (a: number, b: number) => void;
  readonly __wbg_get_registerssnapshot_V3: (a: number) => number;
  readonly __wbg_set_registerssnapshot_V3: (a: number, b: number) => void;
  readonly __wbg_get_registerssnapshot_V4: (a: number) => number;
  readonly __wbg_set_registerssnapshot_V4: (a: number, b: number) => void;
  readonly __wbg_get_registerssnapshot_V5: (a: number) => number;
  readonly __wbg_set_registerssnapshot_V5: (a: number, b: number) => void;
  readonly __wbg_get_registerssnapshot_V6: (a: number) => number;
  readonly __wbg_set_registerssnapshot_V6: (a: number, b: number) => void;
  readonly __wbg_get_registerssnapshot_V7: (a: number) => number;
  readonly __wbg_set_registerssnapshot_V7: (a: number, b: number) => void;
  readonly __wbg_get_registerssnapshot_V8: (a: number) => number;
  readonly __wbg_set_registerssnapshot_V8: (a: number, b: number) => void;
  readonly __wbg_get_registerssnapshot_V9: (a: number) => number;
  readonly __wbg_set_registerssnapshot_V9: (a: number, b: number) => void;
  readonly __wbg_get_registerssnapshot_VA: (a: number) => number;
  readonly __wbg_set_registerssnapshot_VA: (a: number, b: number) => void;
  readonly __wbg_get_registerssnapshot_VB: (a: number) => number;
  readonly __wbg_set_registerssnapshot_VB: (a: number, b: number) => void;
  readonly __wbg_get_registerssnapshot_VC: (a: number) => number;
  readonly __wbg_set_registerssnapshot_VC: (a: number, b: number) => void;
  readonly __wbg_get_registerssnapshot_VD: (a: number) => number;
  readonly __wbg_set_registerssnapshot_VD: (a: number, b: number) => void;
  readonly __wbg_get_registerssnapshot_VE: (a: number) => number;
  readonly __wbg_set_registerssnapshot_VE: (a: number, b: number) => void;
  readonly __wbg_get_registerssnapshot_VF: (a: number) => number;
  readonly __wbg_set_registerssnapshot_VF: (a: number, b: number) => void;
  readonly __wbg_get_registerssnapshot_I: (a: number) => number;
  readonly __wbg_set_registerssnapshot_I: (a: number, b: number) => void;
  readonly __wbg_get_registerssnapshot_delay_timer: (a: number) => number;
  readonly __wbg_set_registerssnapshot_delay_timer: (a: number, b: number) => void;
  readonly __wbg_get_registerssnapshot_sound_timer: (a: number) => number;
  readonly __wbg_set_registerssnapshot_sound_timer: (a: number, b: number) => void;
  readonly __wbg_get_registerssnapshot_programm_counter: (a: number) => number;
  readonly __wbg_set_registerssnapshot_programm_counter: (a: number, b: number) => void;
  readonly __wbg_get_registerssnapshot_stack_pointer: (a: number) => number;
  readonly __wbg_set_registerssnapshot_stack_pointer: (a: number, b: number) => void;
  readonly emulator_new: () => number;
  readonly emulator_set_key: (a: number, b: number, c: number) => void;
  readonly emulator_get_display_ptr: (a: number) => number;
  readonly emulator_get_display_width: (a: number) => number;
  readonly emulator_get_display_height: (a: number) => number;
  readonly emulator_execute_instruction: (a: number) => number;
  readonly emulator_update_timers: (a: number, b: number) => void;
  readonly emulator_is_sound_active: (a: number) => number;
  readonly emulator_load_rom: (a: number, b: number, c: number) => void;
  readonly emulator_get_register_snapshot: (a: number) => number;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __externref_table_alloc: () => number;
  readonly __wbindgen_export_2: WebAssembly.Table;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
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
