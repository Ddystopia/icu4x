// generated by diplomat-tool
import type { DataError } from "./DataError"
import type { DataProvider } from "./DataProvider"
import type { pointer, codepoint } from "./diplomat-runtime.d.ts";


/** An ICU4X Unicode Set Property object, capable of querying whether a code point is contained in a set based on a Unicode property.
*
*See the [Rust documentation for `properties`](https://docs.rs/icu/latest/icu/properties/index.html) for more information.
*
*See the [Rust documentation for `EmojiSetData`](https://docs.rs/icu/latest/icu/properties/struct.EmojiSetData.html) for more information.
*
*See the [Rust documentation for `new`](https://docs.rs/icu/latest/icu/properties/struct.EmojiSetData.html#method.new) for more information.
*
*See the [Rust documentation for `EmojiSetDataBorrowed`](https://docs.rs/icu/latest/icu/properties/struct.EmojiSetDataBorrowed.html) for more information.
*/


export class EmojiSetData {
    
    get ffiValue(): pointer;

    containsStr(s: string): boolean;

    contains(cp: codepoint): boolean;

    static createBasic(): EmojiSetData;

    static createBasicWithProvider(provider: DataProvider): EmojiSetData;
}