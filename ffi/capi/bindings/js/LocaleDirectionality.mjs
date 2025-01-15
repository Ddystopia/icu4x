// generated by diplomat-tool
import { DataError } from "./DataError.mjs"
import { DataProvider } from "./DataProvider.mjs"
import { Locale } from "./Locale.mjs"
import { LocaleDirection } from "./LocaleDirection.mjs"
import wasm from "./diplomat-wasm.mjs";
import * as diplomatRuntime from "./diplomat-runtime.mjs";


/** See the [Rust documentation for `LocaleDirectionality`](https://docs.rs/icu/latest/icu/locale/struct.LocaleDirectionality.html) for more information.
*/
const LocaleDirectionality_box_destroy_registry = new FinalizationRegistry((ptr) => {
    wasm.icu4x_LocaleDirectionality_destroy_mv1(ptr);
});

export class LocaleDirectionality {
    
    // Internal ptr reference:
    #ptr = null;

    // Lifetimes are only to keep dependencies alive.
    // Since JS won't garbage collect until there are no incoming edges.
    #selfEdge = [];
    
    #internalConstructor(symbol, ptr, selfEdge) {
        if (symbol !== diplomatRuntime.internalConstructor) {
            console.error("LocaleDirectionality is an Opaque type. You cannot call its constructor.");
            return;
        }
        
        this.#ptr = ptr;
        this.#selfEdge = selfEdge;
        
        // Are we being borrowed? If not, we can register.
        if (this.#selfEdge.length === 0) {
            LocaleDirectionality_box_destroy_registry.register(this, this.#ptr);
        }
        
        return this;
    }
    get ffiValue() {
        return this.#ptr;
    }

    #defaultConstructor() {
        const result = wasm.icu4x_LocaleDirectionality_create_common_mv1();
    
        try {
            return new LocaleDirectionality(diplomatRuntime.internalConstructor, result, []);
        }
        
        finally {}
    }

    static createCommonWithProvider(provider) {
        const diplomatReceive = new diplomatRuntime.DiplomatReceiveBuf(wasm, 5, 4, true);
        
        const result = wasm.icu4x_LocaleDirectionality_create_common_with_provider_mv1(diplomatReceive.buffer, provider.ffiValue);
    
        try {
            if (!diplomatReceive.resultFlag) {
                const cause = new DataError(diplomatRuntime.internalConstructor, diplomatRuntime.enumDiscriminant(wasm, diplomatReceive.buffer));
                throw new globalThis.Error('DataError: ' + cause.value, { cause });
            }
            return new LocaleDirectionality(diplomatRuntime.internalConstructor, diplomatRuntime.ptrRead(wasm, diplomatReceive.buffer), []);
        }
        
        finally {
            diplomatReceive.free();
        }
    }

    static createExtended() {
        const result = wasm.icu4x_LocaleDirectionality_create_extended_mv1();
    
        try {
            return new LocaleDirectionality(diplomatRuntime.internalConstructor, result, []);
        }
        
        finally {}
    }

    static createExtendedWithProvider(provider) {
        const diplomatReceive = new diplomatRuntime.DiplomatReceiveBuf(wasm, 5, 4, true);
        
        const result = wasm.icu4x_LocaleDirectionality_create_extended_with_provider_mv1(diplomatReceive.buffer, provider.ffiValue);
    
        try {
            if (!diplomatReceive.resultFlag) {
                const cause = new DataError(diplomatRuntime.internalConstructor, diplomatRuntime.enumDiscriminant(wasm, diplomatReceive.buffer));
                throw new globalThis.Error('DataError: ' + cause.value, { cause });
            }
            return new LocaleDirectionality(diplomatRuntime.internalConstructor, diplomatRuntime.ptrRead(wasm, diplomatReceive.buffer), []);
        }
        
        finally {
            diplomatReceive.free();
        }
    }

    get(locale) {
        const result = wasm.icu4x_LocaleDirectionality_get_mv1(this.ffiValue, locale.ffiValue);
    
        try {
            return new LocaleDirection(diplomatRuntime.internalConstructor, result);
        }
        
        finally {}
    }

    isLeftToRight(locale) {
        const result = wasm.icu4x_LocaleDirectionality_is_left_to_right_mv1(this.ffiValue, locale.ffiValue);
    
        try {
            return result;
        }
        
        finally {}
    }

    isRightToLeft(locale) {
        const result = wasm.icu4x_LocaleDirectionality_is_right_to_left_mv1(this.ffiValue, locale.ffiValue);
    
        try {
            return result;
        }
        
        finally {}
    }

    constructor() {
        if (arguments[0] === diplomatRuntime.exposeConstructor) {
            return this.#internalConstructor(...Array.prototype.slice.call(arguments, 1));
        } else if (arguments[0] === diplomatRuntime.internalConstructor) {
            return this.#internalConstructor(...arguments);
        } else {
            return this.#defaultConstructor(...arguments);
        }
    }
}