/* tslint:disable */
/* eslint-disable */
/**
*/
export function main_js(): void;
/**
* @param {string} noise_type
* @param {number} fade_in
* @param {number} hold
* @param {number} fade_out
* @returns {Handle}
*/
export function play_noise(noise_type: string, fade_in: number, hold: number, fade_out: number): Handle;
/**
*/
export class Handle {
  free(): void;
}
