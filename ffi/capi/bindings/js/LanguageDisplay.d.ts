// generated by diplomat-tool
import type { pointer, codepoint } from "./diplomat-runtime.d.ts";


/** See the [Rust documentation for `LanguageDisplay`](https://docs.rs/icu/latest/icu/displaynames/options/enum.LanguageDisplay.html) for more information.
*/


export class LanguageDisplay {
    

    static fromValue(value : LanguageDisplay | string) : LanguageDisplay; 

    get value() : string;

    get ffiValue() : number;

    static Dialect : LanguageDisplay;
    static Standard : LanguageDisplay;

    constructor(value: LanguageDisplay | string );
}