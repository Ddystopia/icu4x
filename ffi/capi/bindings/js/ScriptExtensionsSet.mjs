// generated by diplomat-tool
import wasm from "./diplomat-wasm.mjs";
import * as diplomatRuntime from "./diplomat-runtime.mjs";


/** An object that represents the Script_Extensions property for a single character
*
*See the [Rust documentation for `ScriptExtensionsSet`](https://docs.rs/icu/latest/icu/properties/script/struct.ScriptExtensionsSet.html) for more information.
*/
const ScriptExtensionsSet_box_destroy_registry = new FinalizationRegistry((ptr) => {
    wasm.icu4x_ScriptExtensionsSet_destroy_mv1(ptr);
});

export class ScriptExtensionsSet {
    
    // Internal ptr reference:
    #ptr = null;

    // Lifetimes are only to keep dependencies alive.
    // Since JS won't garbage collect until there are no incoming edges.
    #selfEdge = [];
    #aEdge = [];
    
    #internalConstructor(symbol, ptr, selfEdge, aEdge) {
        if (symbol !== diplomatRuntime.internalConstructor) {
            console.error("ScriptExtensionsSet is an Opaque type. You cannot call its constructor.");
            return;
        }
        
        
        this.#aEdge = aEdge;
        
        this.#ptr = ptr;
        this.#selfEdge = selfEdge;
        
        // Are we being borrowed? If not, we can register.
        if (this.#selfEdge.length === 0) {
            ScriptExtensionsSet_box_destroy_registry.register(this, this.#ptr);
        }
        
        return this;
    }
    get ffiValue() {
        return this.#ptr;
    }

    contains(script) {
        const result = wasm.icu4x_ScriptExtensionsSet_contains_mv1(this.ffiValue, script);
    
        try {
            return result;
        }
        
        finally {}
    }

    get count() {
        const result = wasm.icu4x_ScriptExtensionsSet_count_mv1(this.ffiValue);
    
        try {
            return result;
        }
        
        finally {}
    }

    scriptAt(index) {
        const diplomatReceive = new diplomatRuntime.DiplomatReceiveBuf(wasm, 3, 2, true);
        
        const result = wasm.icu4x_ScriptExtensionsSet_script_at_mv1(diplomatReceive.buffer, this.ffiValue, index);
    
        try {
            if (!diplomatReceive.resultFlag) {
                return null;
            }
            return (new Uint16Array(wasm.memory.buffer, diplomatReceive.buffer, 1))[0];
        }
        
        finally {
            diplomatReceive.free();
        }
    }

    constructor(symbol, ptr, selfEdge, aEdge) {
        return this.#internalConstructor(...arguments)
    }
}