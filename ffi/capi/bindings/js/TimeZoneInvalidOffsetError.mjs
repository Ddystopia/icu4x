// generated by diplomat-tool
import wasm from "./diplomat-wasm.mjs";
import * as diplomatRuntime from "./diplomat-runtime.mjs";


/** Additional information: [1](https://docs.rs/icu/latest/icu/timezone/struct.InvalidOffsetError.html)
*/


export class TimeZoneInvalidOffsetError {
    
    /** Create `TimeZoneInvalidOffsetError` from an object that contains all of `TimeZoneInvalidOffsetError`s fields.
    * Optional fields do not need to be included in the provided object.
    */
    static fromFields(structObj) {
        return new TimeZoneInvalidOffsetError(structObj);
    }
    
    #internalConstructor(structObj) {
        if (typeof structObj !== "object") {
            throw new Error("TimeZoneInvalidOffsetError's constructor takes an object of TimeZoneInvalidOffsetError's fields.");
        }

        return this;
    }


    constructor(structObj) {
        return this.#internalConstructor(...arguments)
    }
}