// generated by diplomat-tool
import type { pointer, codepoint } from "./diplomat-runtime.d.ts";


/** See the [Rust documentation for `IndicSyllabicCategory`](https://docs.rs/icu/latest/icu/properties/props/struct.IndicSyllabicCategory.html) for more information.
*/


export class IndicSyllabicCategory {
    

    static fromValue(value : IndicSyllabicCategory | string) : IndicSyllabicCategory; 

    get value() : string;

    get ffiValue() : number;

    static Other : IndicSyllabicCategory;
    static Avagraha : IndicSyllabicCategory;
    static Bindu : IndicSyllabicCategory;
    static BrahmiJoiningNumber : IndicSyllabicCategory;
    static CantillationMark : IndicSyllabicCategory;
    static Consonant : IndicSyllabicCategory;
    static ConsonantDead : IndicSyllabicCategory;
    static ConsonantFinal : IndicSyllabicCategory;
    static ConsonantHeadLetter : IndicSyllabicCategory;
    static ConsonantInitialPostfixed : IndicSyllabicCategory;
    static ConsonantKiller : IndicSyllabicCategory;
    static ConsonantMedial : IndicSyllabicCategory;
    static ConsonantPlaceholder : IndicSyllabicCategory;
    static ConsonantPrecedingRepha : IndicSyllabicCategory;
    static ConsonantPrefixed : IndicSyllabicCategory;
    static ConsonantSucceedingRepha : IndicSyllabicCategory;
    static ConsonantSubjoined : IndicSyllabicCategory;
    static ConsonantWithStacker : IndicSyllabicCategory;
    static GeminationMark : IndicSyllabicCategory;
    static InvisibleStacker : IndicSyllabicCategory;
    static Joiner : IndicSyllabicCategory;
    static ModifyingLetter : IndicSyllabicCategory;
    static NonJoiner : IndicSyllabicCategory;
    static Nukta : IndicSyllabicCategory;
    static Number : IndicSyllabicCategory;
    static NumberJoiner : IndicSyllabicCategory;
    static PureKiller : IndicSyllabicCategory;
    static RegisterShifter : IndicSyllabicCategory;
    static SyllableModifier : IndicSyllabicCategory;
    static ToneLetter : IndicSyllabicCategory;
    static ToneMark : IndicSyllabicCategory;
    static Virama : IndicSyllabicCategory;
    static Visarga : IndicSyllabicCategory;
    static Vowel : IndicSyllabicCategory;
    static VowelDependent : IndicSyllabicCategory;
    static VowelIndependent : IndicSyllabicCategory;
    static ReorderingKiller : IndicSyllabicCategory;

    toInteger(): number;

    static fromInteger(other: number): IndicSyllabicCategory | null;

    constructor(value: IndicSyllabicCategory | string );
}