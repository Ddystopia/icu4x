// generated by diplomat-tool
import type { DataError } from "./DataError"
import type { DataProvider } from "./DataProvider"
import type { pointer, codepoint } from "./diplomat-runtime.d.ts";


/** A type capable of looking up a property value from a string name.
*
*See the [Rust documentation for `PropertyParser`](https://docs.rs/icu/latest/icu/properties/struct.PropertyParser.html) for more information.
*
*See the [Rust documentation for `PropertyParserBorrowed`](https://docs.rs/icu/latest/icu/properties/struct.PropertyParserBorrowed.html) for more information.
*
*See the [Rust documentation for `new`](https://docs.rs/icu/latest/icu/properties/struct.PropertyParser.html#method.new) for more information.
*/


export class PropertyValueNameToEnumMapper {
    
    get ffiValue(): pointer;

    getStrict(name: string): number;

    getLoose(name: string): number;

    static createGeneralCategory(): PropertyValueNameToEnumMapper;

    static createGeneralCategoryWithProvider(provider: DataProvider): PropertyValueNameToEnumMapper;

    static createHangulSyllableType(): PropertyValueNameToEnumMapper;

    static createHangulSyllableTypeWithProvider(provider: DataProvider): PropertyValueNameToEnumMapper;

    static createEastAsianWidth(): PropertyValueNameToEnumMapper;

    static createEastAsianWidthWithProvider(provider: DataProvider): PropertyValueNameToEnumMapper;

    static createBidiClass(): PropertyValueNameToEnumMapper;

    static createBidiClassWithProvider(provider: DataProvider): PropertyValueNameToEnumMapper;

    static createIndicSyllabicCategory(): PropertyValueNameToEnumMapper;

    static createIndicSyllabicCategoryWithProvider(provider: DataProvider): PropertyValueNameToEnumMapper;

    static createLineBreak(): PropertyValueNameToEnumMapper;

    static createLineBreakWithProvider(provider: DataProvider): PropertyValueNameToEnumMapper;

    static createGraphemeClusterBreak(): PropertyValueNameToEnumMapper;

    static createGraphemeClusterBreakWithProvider(provider: DataProvider): PropertyValueNameToEnumMapper;

    static createWordBreak(): PropertyValueNameToEnumMapper;

    static createWordBreakWithProvider(provider: DataProvider): PropertyValueNameToEnumMapper;

    static createSentenceBreak(): PropertyValueNameToEnumMapper;

    static createSentenceBreakWithProvider(provider: DataProvider): PropertyValueNameToEnumMapper;

    static createScript(): PropertyValueNameToEnumMapper;

    static createScriptWithProvider(provider: DataProvider): PropertyValueNameToEnumMapper;
}