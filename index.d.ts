/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

export type JsNdArray = NdArray
export declare class NdArray {
  constructor(shapeOrData: unknown)
  get shape(): Array<number>
  get size(): number
  get ndim(): number
  get dtype(): string
  get(indices: Array<number>): number
  add(other: NdArray): NdArray
  addScalar(scalar: number): NdArray
  chain(): NdArray
}
