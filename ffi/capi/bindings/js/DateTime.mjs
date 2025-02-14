// generated by diplomat-tool
import { Calendar } from "./Calendar.mjs"
import { CalendarParseError } from "./CalendarParseError.mjs"
import { Date } from "./Date.mjs"
import { Time } from "./Time.mjs"
import wasm from "./diplomat-wasm.mjs";
import * as diplomatRuntime from "./diplomat-runtime.mjs";


/** An ICU4X DateTime object capable of containing a date and time for any calendar.
*
*See the [Rust documentation for `DateTime`](https://docs.rs/icu/latest/icu/time/struct.DateTime.html) for more information.
*/


export class DateTime {
    
    #date;
    
    get date()  {
        return this.#date;
    }
    
    #time;
    
    get time()  {
        return this.#time;
    }
    
    #internalConstructor(structObj, internalConstructor) {
        if (typeof structObj !== "object") {
            throw new Error("DateTime's constructor takes an object of DateTime's fields.");
        }

        if (internalConstructor !== diplomatRuntime.internalConstructor) {
            throw new Error("DateTime is an out struct and can only be created internally.");
        }
        if ("date" in structObj) {
            this.#date = structObj.date;
        } else {
            throw new Error("Missing required field date.");
        }

        if ("time" in structObj) {
            this.#time = structObj.time;
        } else {
            throw new Error("Missing required field time.");
        }

        return this;
    }

    // Return this struct in FFI function friendly format.
    // Returns an array that can be expanded with spread syntax (...)
    
    _intoFFI(
        functionCleanupArena,
        appendArrayMap
    ) {
        return [this.#date.ffiValue, this.#time.ffiValue]
    }

    static _fromSuppliedValue(internalConstructor, obj) {
        if (internalConstructor !== diplomatRuntime.internalConstructor) {
            throw new Error("_fromSuppliedValue cannot be called externally.");
        }

        if (obj instanceof DateTime) {
            return obj;
        }

        return DateTime.fromFields(obj);
    }

    _writeToArrayBuffer(
        arrayBuffer,
        offset,
        functionCleanupArena,
        appendArrayMap
    ) {
        diplomatRuntime.writeToArrayBuffer(arrayBuffer, offset + 0, this.#date.ffiValue, Uint32Array);
        diplomatRuntime.writeToArrayBuffer(arrayBuffer, offset + 4, this.#time.ffiValue, Uint32Array);
    }

    // This struct contains borrowed fields, so this takes in a list of
    // "edges" corresponding to where each lifetime's data may have been borrowed from
    // and passes it down to individual fields containing the borrow.
    // This method does not attempt to handle any dependencies between lifetimes, the caller
    // should handle this when constructing edge arrays.
    static _fromFFI(internalConstructor, ptr) {
        if (internalConstructor !== diplomatRuntime.internalConstructor) {
            throw new Error("DateTime._fromFFI is not meant to be called externally. Please use the default constructor.");
        }
        let structObj = {};
        const dateDeref = diplomatRuntime.ptrRead(wasm, ptr);
        structObj.date = new Date(diplomatRuntime.internalConstructor, dateDeref, []);
        const timeDeref = diplomatRuntime.ptrRead(wasm, ptr + 4);
        structObj.time = new Time(diplomatRuntime.internalConstructor, timeDeref, []);

        return new DateTime(structObj, internalConstructor);
    }

    static fromString(v, calendar) {
        let functionCleanupArena = new diplomatRuntime.CleanupArena();
        
        const vSlice = functionCleanupArena.alloc(diplomatRuntime.DiplomatBuf.str8(wasm, v));
        
        const diplomatReceive = new diplomatRuntime.DiplomatReceiveBuf(wasm, 9, 4, true);
        
        const result = wasm.icu4x_DateTime_from_string_mv1(diplomatReceive.buffer, ...vSlice.splat(), calendar.ffiValue);
    
        try {
            if (!diplomatReceive.resultFlag) {
                const cause = new CalendarParseError(diplomatRuntime.internalConstructor, diplomatRuntime.enumDiscriminant(wasm, diplomatReceive.buffer));
                throw new globalThis.Error('CalendarParseError: ' + cause.value, { cause });
            }
            return DateTime._fromFFI(diplomatRuntime.internalConstructor, diplomatReceive.buffer);
        }
        
        finally {
            functionCleanupArena.free();
        
            diplomatReceive.free();
        }
    }

    constructor(structObj, internalConstructor) {
        return this.#internalConstructor(...arguments)
    }
}