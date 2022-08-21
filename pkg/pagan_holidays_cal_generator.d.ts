/* tslint:disable */
/* eslint-disable */
/**
* @param {CalendarOptions} opts
* @returns {string}
*/
export function make_ics(opts: CalendarOptions): string;
/**
*/
export class CalendarOptions {
  free(): void;
/**
* @param {string} name
* @param {number} start
* @param {number} end
*/
  constructor(name: string, start: number, end: number);
/**
*/
  cross_quarter_days: Uint8Array;
/**
*/
  cross_quarter_days_fmt: string;
/**
*/
  custom_names: any[];
/**
*/
  description?: string;
/**
*/
  name: string;
/**
*/
  quarter_days: Uint8Array;
/**
*/
  quarter_days_fmt: string;
/**
*/
  year_end: number;
/**
*/
  year_start: number;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_calendaroptions_free: (a: number) => void;
  readonly __wbg_get_calendaroptions_year_start: (a: number) => number;
  readonly __wbg_set_calendaroptions_year_start: (a: number, b: number) => void;
  readonly __wbg_get_calendaroptions_year_end: (a: number) => number;
  readonly __wbg_set_calendaroptions_year_end: (a: number, b: number) => void;
  readonly calendaroptions_new: (a: number, b: number, c: number, d: number) => number;
  readonly calendaroptions_name: (a: number, b: number) => void;
  readonly calendaroptions_set_name: (a: number, b: number, c: number) => void;
  readonly calendaroptions_description: (a: number, b: number) => void;
  readonly calendaroptions_set_description: (a: number, b: number, c: number) => void;
  readonly calendaroptions_quarter_days: (a: number, b: number) => void;
  readonly calendaroptions_set_quarter_days: (a: number, b: number, c: number) => void;
  readonly calendaroptions_quarter_days_fmt: (a: number, b: number) => void;
  readonly calendaroptions_set_quarter_days_fmt: (a: number, b: number, c: number) => void;
  readonly calendaroptions_cross_quarter_days: (a: number, b: number) => void;
  readonly calendaroptions_set_cross_quarter_days: (a: number, b: number, c: number) => void;
  readonly calendaroptions_cross_quarter_days_fmt: (a: number, b: number) => void;
  readonly calendaroptions_set_cross_quarter_days_fmt: (a: number, b: number, c: number) => void;
  readonly calendaroptions_custom_names: (a: number, b: number) => void;
  readonly calendaroptions_set_custom_names: (a: number, b: number, c: number) => void;
  readonly make_ics: (a: number, b: number) => void;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_free: (a: number, b: number) => void;
  readonly __wbindgen_exn_store: (a: number) => void;
}

/**
* Synchronously compiles the given `bytes` and instantiates the WebAssembly module.
*
* @param {BufferSource} bytes
*
* @returns {InitOutput}
*/
export function initSync(bytes: BufferSource): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
