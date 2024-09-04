// generated by diplomat-tool
import { CustomTimeZone } from "./CustomTimeZone.mjs"
import { DataProvider } from "./DataProvider.mjs"
import { DateLength } from "./DateLength.mjs"
import { DateTime } from "./DateTime.mjs"
import { Error } from "./Error.mjs"
import { IsoDateTime } from "./IsoDateTime.mjs"
import { IsoTimeZoneOptions } from "./IsoTimeZoneOptions.mjs"
import { Locale } from "./Locale.mjs"
import { TimeLength } from "./TimeLength.mjs"
import wasm from "./diplomat-wasm.mjs";
import * as diplomatRuntime from "./diplomat-runtime.mjs";


/** An object capable of formatting a date time with time zone to a string.
*
*See the [Rust documentation for `ZonedDateTimeFormatter`](https://docs.rs/icu/latest/icu/datetime/struct.ZonedDateTimeFormatter.html) for more information.
*/
const ZonedDateTimeFormatter_box_destroy_registry = new FinalizationRegistry((ptr) => {
    wasm.icu4x_ZonedDateTimeFormatter_destroy_mv1(ptr);
});

export class ZonedDateTimeFormatter {
    // Internal ptr reference:
    #ptr = null;

    // Lifetimes are only to keep dependencies alive.
    // Since JS won't garbage collect until there are no incoming edges.
    #selfEdge = [];
    
    constructor(symbol, ptr, selfEdge) {
        if (symbol !== diplomatRuntime.internalConstructor) {
            console.error("ZonedDateTimeFormatter is an Opaque type. You cannot call its constructor.");
            return;
        }
        
        this.#ptr = ptr;
        this.#selfEdge = selfEdge;
        
        // Are we being borrowed? If not, we can register.
        if (this.#selfEdge.length === 0) {
            ZonedDateTimeFormatter_box_destroy_registry.register(this, this.#ptr);
        }
    }

    get ffiValue() {
        return this.#ptr;
    }

    static createWithLengths(provider, locale, dateLength, timeLength) {
        const diplomatReceive = new diplomatRuntime.DiplomatReceiveBuf(wasm, 5, 4, true);
        
        const result = wasm.icu4x_ZonedDateTimeFormatter_create_with_lengths_mv1(diplomatReceive.buffer, provider.ffiValue, locale.ffiValue, ...diplomatRuntime.optionToArgsForCalling(dateLength, 4, 4, false, (arrayBuffer, offset, jsValue) => [diplomatRuntime.writeToArrayBuffer(arrayBuffer, offset + 0, jsValue.ffiValue, Int32Array)]), ...diplomatRuntime.optionToArgsForCalling(timeLength, 4, 4, false, (arrayBuffer, offset, jsValue) => [diplomatRuntime.writeToArrayBuffer(arrayBuffer, offset + 0, jsValue.ffiValue, Int32Array)]));
    
        try {
            if (!diplomatReceive.resultFlag) {
                const cause = new Error(diplomatRuntime.internalConstructor, diplomatRuntime.enumDiscriminant(wasm, diplomatReceive.buffer));
                throw new globalThis.Error('Error: ' + cause.value, { cause });
            }
            return new ZonedDateTimeFormatter(diplomatRuntime.internalConstructor, diplomatRuntime.ptrRead(wasm, diplomatReceive.buffer), []);
        }
        
        finally {
            diplomatReceive.free();
        }
    }

    static createWithLengthsAndIso8601TimeZoneFallback(provider, locale, dateLength, timeLength, zoneOptions) {
        let functionCleanupArena = new diplomatRuntime.CleanupArena();
        
        const diplomatReceive = new diplomatRuntime.DiplomatReceiveBuf(wasm, 5, 4, true);
        
        const result = wasm.icu4x_ZonedDateTimeFormatter_create_with_lengths_and_iso_8601_time_zone_fallback_mv1(diplomatReceive.buffer, provider.ffiValue, locale.ffiValue, ...diplomatRuntime.optionToArgsForCalling(dateLength, 4, 4, false, (arrayBuffer, offset, jsValue) => [diplomatRuntime.writeToArrayBuffer(arrayBuffer, offset + 0, jsValue.ffiValue, Int32Array)]), ...diplomatRuntime.optionToArgsForCalling(timeLength, 4, 4, false, (arrayBuffer, offset, jsValue) => [diplomatRuntime.writeToArrayBuffer(arrayBuffer, offset + 0, jsValue.ffiValue, Int32Array)]), ...zoneOptions._intoFFI(functionCleanupArena, {}));
    
        try {
            if (!diplomatReceive.resultFlag) {
                const cause = new Error(diplomatRuntime.internalConstructor, diplomatRuntime.enumDiscriminant(wasm, diplomatReceive.buffer));
                throw new globalThis.Error('Error: ' + cause.value, { cause });
            }
            return new ZonedDateTimeFormatter(diplomatRuntime.internalConstructor, diplomatRuntime.ptrRead(wasm, diplomatReceive.buffer), []);
        }
        
        finally {
            functionCleanupArena.free();
        
            diplomatReceive.free();
        }
    }

    formatDatetimeWithCustomTimeZone(datetime, timeZone) {
        const diplomatReceive = new diplomatRuntime.DiplomatReceiveBuf(wasm, 5, 4, true);
        
        const write = new diplomatRuntime.DiplomatWriteBuf(wasm);
        
        const result = wasm.icu4x_ZonedDateTimeFormatter_format_datetime_with_custom_time_zone_mv1(diplomatReceive.buffer, this.ffiValue, datetime.ffiValue, timeZone.ffiValue, write.buffer);
    
        try {
            if (!diplomatReceive.resultFlag) {
                const cause = new Error(diplomatRuntime.internalConstructor, diplomatRuntime.enumDiscriminant(wasm, diplomatReceive.buffer));
                throw new globalThis.Error('Error: ' + cause.value, { cause });
            }
            return write.readString8();
        }
        
        finally {
            diplomatReceive.free();
        
            write.free();
        }
    }

    formatIsoDatetimeWithCustomTimeZone(datetime, timeZone) {
        const diplomatReceive = new diplomatRuntime.DiplomatReceiveBuf(wasm, 5, 4, true);
        
        const write = new diplomatRuntime.DiplomatWriteBuf(wasm);
        
        const result = wasm.icu4x_ZonedDateTimeFormatter_format_iso_datetime_with_custom_time_zone_mv1(diplomatReceive.buffer, this.ffiValue, datetime.ffiValue, timeZone.ffiValue, write.buffer);
    
        try {
            if (!diplomatReceive.resultFlag) {
                const cause = new Error(diplomatRuntime.internalConstructor, diplomatRuntime.enumDiscriminant(wasm, diplomatReceive.buffer));
                throw new globalThis.Error('Error: ' + cause.value, { cause });
            }
            return write.readString8();
        }
        
        finally {
            diplomatReceive.free();
        
            write.free();
        }
    }
}