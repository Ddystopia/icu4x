// generated by diplomat-tool
import type { DataError } from "./DataError"
import type { DataProvider } from "./DataProvider"
import type { LineBreakIteratorUtf16 } from "./LineBreakIteratorUtf16"
import type { LineBreakOptions } from "./LineBreakOptions"
import type { LineBreakOptions_obj } from "./LineBreakOptions"
import type { Locale } from "./Locale"
import type { pointer, codepoint } from "./diplomat-runtime.d.ts";


/** An ICU4X line-break segmenter, capable of finding breakpoints in strings.
*
*See the [Rust documentation for `LineSegmenter`](https://docs.rs/icu/latest/icu/segmenter/struct.LineSegmenter.html) for more information.
*/


export class LineSegmenter {
    
    get ffiValue(): pointer;

    static createAuto(): LineSegmenter;

    static createLstm(): LineSegmenter;

    static createDictionary(): LineSegmenter;

    static autoWithOptions(contentLocale: Locale | null, options: LineBreakOptions_obj): LineSegmenter;

    static autoWithOptionsAndProvider(provider: DataProvider, contentLocale: Locale | null, options: LineBreakOptions_obj): LineSegmenter;

    static lstmWithOptions(contentLocale: Locale | null, options: LineBreakOptions_obj): LineSegmenter;

    static lstmWithOptionsAndProvider(provider: DataProvider, contentLocale: Locale | null, options: LineBreakOptions_obj): LineSegmenter;

    static dictionaryWithOptions(contentLocale: Locale | null, options: LineBreakOptions_obj): LineSegmenter;

    static dictionaryWithOptionsAndProvider(provider: DataProvider, contentLocale: Locale | null, options: LineBreakOptions_obj): LineSegmenter;

    segment(input: string): LineBreakIteratorUtf16;
}