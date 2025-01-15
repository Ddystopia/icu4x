// generated by diplomat-tool
import { DataError } from "./DataError.mjs"
import { DataProvider } from "./DataProvider.mjs"
import { MeasureUnit } from "./MeasureUnit.mjs"
import { MeasureUnitParser } from "./MeasureUnitParser.mjs"
import { UnitsConverter } from "./UnitsConverter.mjs"
import wasm from "./diplomat-wasm.mjs";
import * as diplomatRuntime from "./diplomat-runtime.mjs";


/** An ICU4X Units Converter Factory object, capable of creating converters a [`UnitsConverter`]
*for converting between two [`MeasureUnit`]s.
*
*Also, it can parse the CLDR unit identifier (e.g. `meter-per-square-second`) and get the [`MeasureUnit`].
*
*See the [Rust documentation for `ConverterFactory`](https://docs.rs/icu/latest/icu/experimental/units/converter_factory/struct.ConverterFactory.html) for more information.
*/
const UnitsConverterFactory_box_destroy_registry = new FinalizationRegistry((ptr) => {
    wasm.icu4x_UnitsConverterFactory_destroy_mv1(ptr);
});

export class UnitsConverterFactory {
    
    // Internal ptr reference:
    #ptr = null;

    // Lifetimes are only to keep dependencies alive.
    // Since JS won't garbage collect until there are no incoming edges.
    #selfEdge = [];
    
    #internalConstructor(symbol, ptr, selfEdge) {
        if (symbol !== diplomatRuntime.internalConstructor) {
            console.error("UnitsConverterFactory is an Opaque type. You cannot call its constructor.");
            return;
        }
        
        this.#ptr = ptr;
        this.#selfEdge = selfEdge;
        
        // Are we being borrowed? If not, we can register.
        if (this.#selfEdge.length === 0) {
            UnitsConverterFactory_box_destroy_registry.register(this, this.#ptr);
        }
        
        return this;
    }
    get ffiValue() {
        return this.#ptr;
    }

    #defaultConstructor() {
        const result = wasm.icu4x_UnitsConverterFactory_create_mv1();
    
        try {
            return new UnitsConverterFactory(diplomatRuntime.internalConstructor, result, []);
        }
        
        finally {}
    }

    static createWithProvider(provider) {
        const diplomatReceive = new diplomatRuntime.DiplomatReceiveBuf(wasm, 5, 4, true);
        
        const result = wasm.icu4x_UnitsConverterFactory_create_with_provider_mv1(diplomatReceive.buffer, provider.ffiValue);
    
        try {
            if (!diplomatReceive.resultFlag) {
                const cause = new DataError(diplomatRuntime.internalConstructor, diplomatRuntime.enumDiscriminant(wasm, diplomatReceive.buffer));
                throw new globalThis.Error('DataError: ' + cause.value, { cause });
            }
            return new UnitsConverterFactory(diplomatRuntime.internalConstructor, diplomatRuntime.ptrRead(wasm, diplomatReceive.buffer), []);
        }
        
        finally {
            diplomatReceive.free();
        }
    }

    converter(from, to) {
        const result = wasm.icu4x_UnitsConverterFactory_converter_mv1(this.ffiValue, from.ffiValue, to.ffiValue);
    
        try {
            return result === 0 ? null : new UnitsConverter(diplomatRuntime.internalConstructor, result, []);
        }
        
        finally {}
    }

    parser() {
        // This lifetime edge depends on lifetimes 'a
        let aEdges = [this];
        
        const result = wasm.icu4x_UnitsConverterFactory_parser_mv1(this.ffiValue);
    
        try {
            return new MeasureUnitParser(diplomatRuntime.internalConstructor, result, [], aEdges);
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