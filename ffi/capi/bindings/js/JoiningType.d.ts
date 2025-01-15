// generated by diplomat-tool
import type { pointer, codepoint } from "./diplomat-runtime.d.ts";


/** See the [Rust documentation for `JoiningType`](https://docs.rs/icu/latest/icu/properties/props/struct.JoiningType.html) for more information.
*/


export class JoiningType {
    

    static fromValue(value : JoiningType | string) : JoiningType; 

    get value() : string;

    get ffiValue() : number;

    static NonJoining : JoiningType;
    static JoinCausing : JoiningType;
    static DualJoining : JoiningType;
    static LeftJoining : JoiningType;
    static RightJoining : JoiningType;
    static Transparent : JoiningType;

    toInteger(): number;

    static fromInteger(other: number): JoiningType | null;

    constructor(value: JoiningType | string );
}