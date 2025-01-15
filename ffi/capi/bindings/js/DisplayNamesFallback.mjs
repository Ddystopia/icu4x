// generated by diplomat-tool
import wasm from "./diplomat-wasm.mjs";
import * as diplomatRuntime from "./diplomat-runtime.mjs";


/** See the [Rust documentation for `Fallback`](https://docs.rs/icu/latest/icu/displaynames/options/enum.Fallback.html) for more information.
*/


export class DisplayNamesFallback {
    
    #value = undefined;

    static #values = new Map([
        ["Code", 0],
        ["None", 1]
    ]);

    static getAllEntries() {
        return DisplayNamesFallback.#values.entries();
    }
    
    #internalConstructor(value) {
        if (arguments.length > 1 && arguments[0] === diplomatRuntime.internalConstructor) {
            // We pass in two internalConstructor arguments to create *new*
            // instances of this type, otherwise the enums are treated as singletons.
            if (arguments[1] === diplomatRuntime.internalConstructor ) {
                this.#value = arguments[2];
                return this;
            }
            return DisplayNamesFallback.#objectValues[arguments[1]];
        }

        if (value instanceof DisplayNamesFallback) {
            return value;
        }

        let intVal = DisplayNamesFallback.#values.get(value);

        // Nullish check, checks for null or undefined
        if (intVal != null) {
            return DisplayNamesFallback.#objectValues[intVal];
        }

        throw TypeError(value + " is not a DisplayNamesFallback and does not correspond to any of its enumerator values.");
    }

    static fromValue(value) {
        return new DisplayNamesFallback(value);
    }

    get value() {
        return [...DisplayNamesFallback.#values.keys()][this.#value];
    }

    get ffiValue() {
        return this.#value;
    }
    static #objectValues = [
        new DisplayNamesFallback(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 0),
        new DisplayNamesFallback(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 1),
    ];

    static Code = DisplayNamesFallback.#objectValues[0];
    static None = DisplayNamesFallback.#objectValues[1];

    constructor(value) {
        return this.#internalConstructor(...arguments)
    }
}