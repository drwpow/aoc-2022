/* tslint:disable */
/* eslint-disable */
/**
* @param {string} input
* @returns {any}
*/
export function day01(input: string): any;
/**
* @param {string} input
* @returns {any}
*/
export function day02(input: string): any;
/**
* @param {string} input
* @returns {any}
*/
export function day03(input: string): any;
/**
* @param {string} input
* @returns {any}
*/
export function day04(input: string): any;
/**
* @param {any} input_raw
* @returns {any}
*/
export function day05(input_raw: any): any;
/**
* @param {string} input
* @returns {any}
*/
export function day06(input: string): any;
/**
* @param {string} input
* @returns {any}
*/
export function day07(input: string): any;
/**
* @param {string} input
* @returns {any}
*/
export function day08(input: string): any;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly day01: (a: number, b: number) => number;
  readonly day02: (a: number, b: number) => number;
  readonly day03: (a: number, b: number) => number;
  readonly day04: (a: number, b: number) => number;
  readonly day05: (a: number) => number;
  readonly day06: (a: number, b: number) => number;
  readonly day07: (a: number, b: number) => number;
  readonly day08: (a: number, b: number) => number;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __wbindgen_free: (a: number, b: number) => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {SyncInitInput} module
*
* @returns {InitOutput}
*/
export function initSync(module: SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
