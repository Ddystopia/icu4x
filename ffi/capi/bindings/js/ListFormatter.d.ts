// generated by diplomat-tool
import type { DataError } from "./DataError"
import type { DataProvider } from "./DataProvider"
import type { ListLength } from "./ListLength"
import type { Locale } from "./Locale"
import type { pointer, codepoint } from "./diplomat-runtime.d.ts";


/** See the [Rust documentation for `ListFormatter`](https://docs.rs/icu/latest/icu/list/struct.ListFormatter.html) for more information.
*/


export class ListFormatter {
    
    get ffiValue(): pointer;

    static createAndWithLength(locale: Locale, length: ListLength): ListFormatter;

    static createAndWithLengthAndProvider(provider: DataProvider, locale: Locale, length: ListLength): ListFormatter;

    static createOrWithLength(locale: Locale, length: ListLength): ListFormatter;

    static createOrWithLengthAndProvider(provider: DataProvider, locale: Locale, length: ListLength): ListFormatter;

    static createUnitWithLength(locale: Locale, length: ListLength): ListFormatter;

    static createUnitWithLengthAndProvider(provider: DataProvider, locale: Locale, length: ListLength): ListFormatter;

    format(list: Array<string>): string;
}