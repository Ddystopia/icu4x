// generated by diplomat-tool
import wasm from "./diplomat-wasm.mjs";
import * as diplomatRuntime from "./diplomat-runtime.mjs";

// Base enumerator definition
/** Legacy error
*
*Additional information: [1](https://docs.rs/icu/latest/icu/datetime/enum.DateTimeError.html), [2](https://docs.rs/icu/latest/icu/provider/struct.DataError.html), [3](https://docs.rs/icu/latest/icu/provider/enum.DataErrorKind.html)
*/
export class Error {
    #value = undefined;

    static #values = new Map([
        ["UnknownError", 0],
        ["DataMissingDataMarkerError", 256],
        ["DataMissingLocaleError", 258],
        ["DataNeedsLocaleError", 260],
        ["DataExtraneousLocaleError", 261],
        ["DataFilteredResourceError", 262],
        ["DataMismatchedTypeError", 263],
        ["DataCustomError", 266],
        ["DataIoError", 267],
        ["DataUnavailableBufferFormatError", 268],
        ["DateTimePatternError", 2048],
        ["DateTimeMissingInputFieldError", 2049],
        ["DateTimeSkeletonError", 2050],
        ["DateTimeUnsupportedFieldError", 2051],
        ["DateTimeUnsupportedOptionsError", 2052],
        ["DateTimeMissingWeekdaySymbolError", 2053],
        ["DateTimeMissingMonthSymbolError", 2054],
        ["DateTimeFixedDecimalError", 2055],
        ["DateTimeMismatchedCalendarError", 2056],
        ["DateTimeDuplicateFieldError", 2057],
        ["DateTimeTooNarrowError", 2058],
        ["DateTimeMissingNamesError", 2059],
        ["DateTimeZoneInfoMissingFieldsError", 2176]
    ]);

    constructor(value) {
        if (arguments.length > 1 && arguments[0] === diplomatRuntime.internalConstructor) {
            // We pass in two internalConstructor arguments to create *new*
            // instances of this type, otherwise the enums are treated as singletons.
            if (arguments[1] === diplomatRuntime.internalConstructor ) {
                this.#value = arguments[2];
                return;
            }
            return Error.#objectValues[arguments[1]];
        }

        if (value instanceof Error) {
            return value;
        }

        let intVal = Error.#values.get(value);

        // Nullish check, checks for null or undefined
        if (intVal == null) {
            return Error.#objectValues[intVal];
        }

        throw TypeError(value + " is not a Error and does not correspond to any of its enumerator values.");
    }

    get value() {
        for (let entry of Error.#values) {
            if (entry[1] == this.#value) {
                return entry[0];
            }
        }
    }

    get ffiValue() {
        return this.#value;
    }
    static #objectValues = {
        [0]: new Error(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 0),
        [256]: new Error(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 256),
        [258]: new Error(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 258),
        [260]: new Error(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 260),
        [261]: new Error(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 261),
        [262]: new Error(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 262),
        [263]: new Error(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 263),
        [266]: new Error(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 266),
        [267]: new Error(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 267),
        [268]: new Error(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 268),
        [2048]: new Error(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 2048),
        [2049]: new Error(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 2049),
        [2050]: new Error(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 2050),
        [2051]: new Error(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 2051),
        [2052]: new Error(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 2052),
        [2053]: new Error(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 2053),
        [2054]: new Error(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 2054),
        [2055]: new Error(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 2055),
        [2056]: new Error(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 2056),
        [2057]: new Error(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 2057),
        [2058]: new Error(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 2058),
        [2059]: new Error(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 2059),
        [2176]: new Error(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 2176),
    };

    static UnknownError = Error.#objectValues[0];
    static DataMissingDataMarkerError = Error.#objectValues[256];
    static DataMissingLocaleError = Error.#objectValues[258];
    static DataNeedsLocaleError = Error.#objectValues[260];
    static DataExtraneousLocaleError = Error.#objectValues[261];
    static DataFilteredResourceError = Error.#objectValues[262];
    static DataMismatchedTypeError = Error.#objectValues[263];
    static DataCustomError = Error.#objectValues[266];
    static DataIoError = Error.#objectValues[267];
    static DataUnavailableBufferFormatError = Error.#objectValues[268];
    static DateTimePatternError = Error.#objectValues[2048];
    static DateTimeMissingInputFieldError = Error.#objectValues[2049];
    static DateTimeSkeletonError = Error.#objectValues[2050];
    static DateTimeUnsupportedFieldError = Error.#objectValues[2051];
    static DateTimeUnsupportedOptionsError = Error.#objectValues[2052];
    static DateTimeMissingWeekdaySymbolError = Error.#objectValues[2053];
    static DateTimeMissingMonthSymbolError = Error.#objectValues[2054];
    static DateTimeFixedDecimalError = Error.#objectValues[2055];
    static DateTimeMismatchedCalendarError = Error.#objectValues[2056];
    static DateTimeDuplicateFieldError = Error.#objectValues[2057];
    static DateTimeTooNarrowError = Error.#objectValues[2058];
    static DateTimeMissingNamesError = Error.#objectValues[2059];
    static DateTimeZoneInfoMissingFieldsError = Error.#objectValues[2176];
}