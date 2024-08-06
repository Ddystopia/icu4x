// generated by diplomat-tool
import type { DataProvider } from "./DataProvider"
import type { ListLength } from "./ListLength"
import type { Locale } from "./Locale"
import type { pointer, char } from "./diplomat-runtime.d.ts";


/** See the [Rust documentation for `ListFormatter`](https://docs.rs/icu/latest/icu/list/struct.ListFormatter.html) for more information.
*/
export class ListFormatter {
    

    get ffiValue(): pointer;


    static createAndWithLength(provider: DataProvider, locale: Locale, length: ListLength): ListFormatter;

    static createOrWithLength(provider: DataProvider, locale: Locale, length: ListLength): ListFormatter;

    static createUnitWithLength(provider: DataProvider, locale: Locale, length: ListLength): ListFormatter;

    format(list: Array<String>): string;

    

}