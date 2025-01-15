// generated by diplomat-tool
import wasm from "./diplomat-wasm.mjs";
import * as diplomatRuntime from "./diplomat-runtime.mjs";


/** See the [Rust documentation for `LanguageDisplay`](https://docs.rs/icu/latest/icu/displaynames/options/enum.LanguageDisplay.html) for more information.
*/


export class LanguageDisplay {
    
    #value = undefined;

    static #values = new Map([
        ["Dialect", 0],
        ["Standard", 1]
    ]);

    static getAllEntries() {
        return LanguageDisplay.#values.entries();
    }
    
    #internalConstructor(value) {
        if (arguments.length > 1 && arguments[0] === diplomatRuntime.internalConstructor) {
            // We pass in two internalConstructor arguments to create *new*
            // instances of this type, otherwise the enums are treated as singletons.
            if (arguments[1] === diplomatRuntime.internalConstructor ) {
                this.#value = arguments[2];
                return this;
            }
            return LanguageDisplay.#objectValues[arguments[1]];
        }

        if (value instanceof LanguageDisplay) {
            return value;
        }

        let intVal = LanguageDisplay.#values.get(value);

        // Nullish check, checks for null or undefined
        if (intVal != null) {
            return LanguageDisplay.#objectValues[intVal];
        }

        throw TypeError(value + " is not a LanguageDisplay and does not correspond to any of its enumerator values.");
    }

    static fromValue(value) {
        return new LanguageDisplay(value);
    }

    get value() {
        return [...LanguageDisplay.#values.keys()][this.#value];
    }

    get ffiValue() {
        return this.#value;
    }
    static #objectValues = [
        new LanguageDisplay(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 0),
        new LanguageDisplay(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 1),
    ];

    static Dialect = LanguageDisplay.#objectValues[0];
    static Standard = LanguageDisplay.#objectValues[1];

    constructor(value) {
        return this.#internalConstructor(...arguments)
    }
}