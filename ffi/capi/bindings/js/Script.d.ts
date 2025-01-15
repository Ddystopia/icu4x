// generated by diplomat-tool
import type { pointer, codepoint } from "./diplomat-runtime.d.ts";


/** See the [Rust documentation for `Script`](https://docs.rs/icu/latest/icu/properties/props/struct.Script.html) for more information.
*/


export class Script {
    

    static fromValue(value : Script | string) : Script; 

    get value() : string;

    get ffiValue() : number;

    static Adlam : Script;
    static Ahom : Script;
    static AnatolianHieroglyphs : Script;
    static Arabic : Script;
    static Armenian : Script;
    static Avestan : Script;
    static Balinese : Script;
    static Bamum : Script;
    static BassaVah : Script;
    static Batak : Script;
    static Bengali : Script;
    static Bhaiksuki : Script;
    static Bopomofo : Script;
    static Brahmi : Script;
    static Braille : Script;
    static Buginese : Script;
    static Buhid : Script;
    static CanadianAboriginal : Script;
    static Carian : Script;
    static CaucasianAlbanian : Script;
    static Chakma : Script;
    static Cham : Script;
    static Cherokee : Script;
    static Chorasmian : Script;
    static Common : Script;
    static Coptic : Script;
    static Cuneiform : Script;
    static Cypriot : Script;
    static CyproMinoan : Script;
    static Cyrillic : Script;
    static Deseret : Script;
    static Devanagari : Script;
    static DivesAkuru : Script;
    static Dogra : Script;
    static Duployan : Script;
    static EgyptianHieroglyphs : Script;
    static Elbasan : Script;
    static Elymaic : Script;
    static Ethiopian : Script;
    static Georgian : Script;
    static Glagolitic : Script;
    static Gothic : Script;
    static Grantha : Script;
    static Greek : Script;
    static Gujarati : Script;
    static GunjalaGondi : Script;
    static Gurmukhi : Script;
    static Han : Script;
    static Hangul : Script;
    static HanifiRohingya : Script;
    static Hanunoo : Script;
    static Hatran : Script;
    static Hebrew : Script;
    static Hiragana : Script;
    static ImperialAramaic : Script;
    static Inherited : Script;
    static InscriptionalPahlavi : Script;
    static InscriptionalParthian : Script;
    static Javanese : Script;
    static Kaithi : Script;
    static Kannada : Script;
    static Katakana : Script;
    static Kawi : Script;
    static KayahLi : Script;
    static Kharoshthi : Script;
    static KhitanSmallScript : Script;
    static Khmer : Script;
    static Khojki : Script;
    static Khudawadi : Script;
    static Lao : Script;
    static Latin : Script;
    static Lepcha : Script;
    static Limbu : Script;
    static LinearA : Script;
    static LinearB : Script;
    static Lisu : Script;
    static Lycian : Script;
    static Lydian : Script;
    static Mahajani : Script;
    static Makasar : Script;
    static Malayalam : Script;
    static Mandaic : Script;
    static Manichaean : Script;
    static Marchen : Script;
    static MasaramGondi : Script;
    static Medefaidrin : Script;
    static MeeteiMayek : Script;
    static MendeKikakui : Script;
    static MeroiticCursive : Script;
    static MeroiticHieroglyphs : Script;
    static Miao : Script;
    static Modi : Script;
    static Mongolian : Script;
    static Mro : Script;
    static Multani : Script;
    static Myanmar : Script;
    static Nabataean : Script;
    static NagMundari : Script;
    static Nandinagari : Script;
    static Nastaliq : Script;
    static NewTaiLue : Script;
    static Newa : Script;
    static Nko : Script;
    static Nushu : Script;
    static NyiakengPuachueHmong : Script;
    static Ogham : Script;
    static OlChiki : Script;
    static OldHungarian : Script;
    static OldItalic : Script;
    static OldNorthArabian : Script;
    static OldPermic : Script;
    static OldPersian : Script;
    static OldSogdian : Script;
    static OldSouthArabian : Script;
    static OldTurkic : Script;
    static OldUyghur : Script;
    static Oriya : Script;
    static Osage : Script;
    static Osmanya : Script;
    static PahawhHmong : Script;
    static Palmyrene : Script;
    static PauCinHau : Script;
    static PhagsPa : Script;
    static Phoenician : Script;
    static PsalterPahlavi : Script;
    static Rejang : Script;
    static Runic : Script;
    static Samaritan : Script;
    static Saurashtra : Script;
    static Sharada : Script;
    static Shavian : Script;
    static Siddham : Script;
    static SignWriting : Script;
    static Sinhala : Script;
    static Sogdian : Script;
    static SoraSompeng : Script;
    static Soyombo : Script;
    static Sundanese : Script;
    static SylotiNagri : Script;
    static Syriac : Script;
    static Tagalog : Script;
    static Tagbanwa : Script;
    static TaiLe : Script;
    static TaiTham : Script;
    static TaiViet : Script;
    static Takri : Script;
    static Tamil : Script;
    static Tangsa : Script;
    static Tangut : Script;
    static Telugu : Script;
    static Thaana : Script;
    static Thai : Script;
    static Tibetan : Script;
    static Tifinagh : Script;
    static Tirhuta : Script;
    static Toto : Script;
    static Ugaritic : Script;
    static Unknown : Script;
    static Vai : Script;
    static Vithkuqi : Script;
    static Wancho : Script;
    static WarangCiti : Script;
    static Yezidi : Script;
    static Yi : Script;
    static ZanabazarSquare : Script;

    toInteger(): number;

    static fromInteger(other: number): Script | null;

    constructor(value: Script | string );
}