// generated by diplomat-tool
import type { BidiInfo } from "./BidiInfo"
import type { DataError } from "./DataError"
import type { DataProvider } from "./DataProvider"
import type { ReorderedIndexMap } from "./ReorderedIndexMap"
import type { pointer, codepoint } from "./diplomat-runtime.d.ts";


/** An ICU4X Bidi object, containing loaded bidi data
*
*See the [Rust documentation for `BidiClassAdapter`](https://docs.rs/icu/latest/icu/properties/bidi/struct.BidiClassAdapter.html) for more information.
*
*See the [Rust documentation for `BidiClass`](https://docs.rs/icu/latest/icu/properties/props/struct.BidiClass.html) for more information.
*/


export class Bidi {
    
    get ffiValue(): pointer;

    static createWithProvider(provider: DataProvider): Bidi;

    forText(text: string, defaultLevel: number | null): BidiInfo;

    reorderVisual(levels: Array<number>): ReorderedIndexMap;

    static levelIsRtl(level: number): boolean;

    static levelIsLtr(level: number): boolean;

    static levelRtl(): number;

    static levelLtr(): number;

    constructor();
}