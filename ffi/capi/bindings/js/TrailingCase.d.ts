// generated by diplomat-tool
import type { pointer, codepoint } from "./diplomat-runtime.d.ts";


/** See the [Rust documentation for `TrailingCase`](https://docs.rs/icu/latest/icu/casemap/titlecase/enum.TrailingCase.html) for more information.
*/


export class TrailingCase {
    

    static fromValue(value : TrailingCase | string) : TrailingCase; 

    get value() : string;

    get ffiValue() : number;

    static Lower : TrailingCase;
    static Unchanged : TrailingCase;

    constructor(value: TrailingCase | string );
}