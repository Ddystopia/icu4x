// generated by diplomat-tool
import type { DataProvider } from "./DataProvider"
import type { pointer, char } from "./diplomat-runtime.d.ts";


/** A type capable of looking up General Category mask values from a string name.
*
*See the [Rust documentation for `name_to_enum_mapper`](https://docs.rs/icu/latest/icu/properties/struct.GeneralCategoryGroup.html#method.name_to_enum_mapper) for more information.
*
*See the [Rust documentation for `PropertyValueNameToEnumMapper`](https://docs.rs/icu/latest/icu/properties/names/struct.PropertyValueNameToEnumMapper.html) for more information.
*/
export class GeneralCategoryNameToMaskMapper {
    

    get ffiValue(): pointer;


    getStrict(name: string): number;

    getLoose(name: string): number;

    static load(provider: DataProvider): GeneralCategoryNameToMaskMapper;

    

}