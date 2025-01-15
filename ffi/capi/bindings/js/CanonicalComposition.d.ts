// generated by diplomat-tool
import type { DataError } from "./DataError"
import type { DataProvider } from "./DataProvider"
import type { pointer, codepoint } from "./diplomat-runtime.d.ts";


/** The raw canonical composition operation.
*
*Callers should generally use ComposingNormalizer unless they specifically need raw composition operations
*
*See the [Rust documentation for `CanonicalComposition`](https://docs.rs/icu/latest/icu/normalizer/properties/struct.CanonicalComposition.html) for more information.
*/


export class CanonicalComposition {
    
    get ffiValue(): pointer;

    static createWithProvider(provider: DataProvider): CanonicalComposition;

    compose(starter: codepoint, second: codepoint): codepoint;

    constructor();
}