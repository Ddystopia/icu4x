// generated by diplomat-tool
import type { pointer, codepoint } from "./diplomat-runtime.d.ts";


/** See the [Rust documentation for `Fallback`](https://docs.rs/icu/latest/icu/displaynames/options/enum.Fallback.html) for more information.
*/


export class DisplayNamesFallback {
    

    static fromValue(value : DisplayNamesFallback | string) : DisplayNamesFallback; 

    get value() : string;

    get ffiValue() : number;

    static Code : DisplayNamesFallback;
    static None : DisplayNamesFallback;

    constructor(value: DisplayNamesFallback | string );
}