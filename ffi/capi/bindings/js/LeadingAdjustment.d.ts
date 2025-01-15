// generated by diplomat-tool
import type { pointer, codepoint } from "./diplomat-runtime.d.ts";


/** See the [Rust documentation for `LeadingAdjustment`](https://docs.rs/icu/latest/icu/casemap/titlecase/enum.LeadingAdjustment.html) for more information.
*/


export class LeadingAdjustment {
    

    static fromValue(value : LeadingAdjustment | string) : LeadingAdjustment; 

    get value() : string;

    get ffiValue() : number;

    static Auto : LeadingAdjustment;
    static None : LeadingAdjustment;
    static ToCased : LeadingAdjustment;

    constructor(value: LeadingAdjustment | string );
}