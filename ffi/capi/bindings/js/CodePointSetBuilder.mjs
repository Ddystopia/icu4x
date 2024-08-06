// generated by diplomat-tool
import { CodePointSetData } from "./CodePointSetData.mjs"
import wasm from "./diplomat-wasm.mjs";
import * as diplomatRuntime from "./diplomat-runtime.mjs";


/** See the [Rust documentation for `CodePointInversionListBuilder`](https://docs.rs/icu/latest/icu/collections/codepointinvlist/struct.CodePointInversionListBuilder.html) for more information.
*/

const CodePointSetBuilder_box_destroy_registry = new FinalizationRegistry((ptr) => {
    wasm.icu4x_CodePointSetBuilder_destroy_mv1(ptr);
});
export class CodePointSetBuilder {
    // Internal ptr reference:
    #ptr = null;

    // Lifetimes are only to keep dependencies alive.
    // Since JS won't garbage collect until there are no incoming edges.
    #selfEdge = [];
    
    
    constructor(ptr, selfEdge) {
        
        this.#ptr = ptr;
        this.#selfEdge = selfEdge;
        // Unconditionally register to destroy when this object is ready to garbage collect.
        CodePointSetBuilder_box_destroy_registry.register(this, this.#ptr);
    }

    get ffiValue() {
        return this.#ptr;
    }


    static create() {
        const result = wasm.icu4x_CodePointSetBuilder_create_mv1();
    
        try {
    
            return new CodePointSetBuilder(result, []);
        } finally {
        
        }
    }

    build() {
        const result = wasm.icu4x_CodePointSetBuilder_build_mv1(this.ffiValue);
    
        try {
    
            return new CodePointSetData(result, []);
        } finally {
        
        }
    }

    complement() {
        wasm.icu4x_CodePointSetBuilder_complement_mv1(this.ffiValue);
    
        try {
    
        } finally {
        
        }
    }

    get isEmpty() {
        const result = wasm.icu4x_CodePointSetBuilder_is_empty_mv1(this.ffiValue);
    
        try {
    
            return result;
        } finally {
        
        }
    }

    addChar(ch) {
        wasm.icu4x_CodePointSetBuilder_add_char_mv1(this.ffiValue, diplomatRuntime.extractCodePoint(ch, 'ch'));
    
        try {
    
        } finally {
        
        }
    }

    addInclusiveRange(start, end) {
        wasm.icu4x_CodePointSetBuilder_add_inclusive_range_mv1(this.ffiValue, diplomatRuntime.extractCodePoint(start, 'start'), diplomatRuntime.extractCodePoint(end, 'end'));
    
        try {
    
        } finally {
        
        }
    }

    addSet(data) {
        wasm.icu4x_CodePointSetBuilder_add_set_mv1(this.ffiValue, data.ffiValue);
    
        try {
    
        } finally {
        
        }
    }

    removeChar(ch) {
        wasm.icu4x_CodePointSetBuilder_remove_char_mv1(this.ffiValue, diplomatRuntime.extractCodePoint(ch, 'ch'));
    
        try {
    
        } finally {
        
        }
    }

    removeInclusiveRange(start, end) {
        wasm.icu4x_CodePointSetBuilder_remove_inclusive_range_mv1(this.ffiValue, diplomatRuntime.extractCodePoint(start, 'start'), diplomatRuntime.extractCodePoint(end, 'end'));
    
        try {
    
        } finally {
        
        }
    }

    removeSet(data) {
        wasm.icu4x_CodePointSetBuilder_remove_set_mv1(this.ffiValue, data.ffiValue);
    
        try {
    
        } finally {
        
        }
    }

    retainChar(ch) {
        wasm.icu4x_CodePointSetBuilder_retain_char_mv1(this.ffiValue, diplomatRuntime.extractCodePoint(ch, 'ch'));
    
        try {
    
        } finally {
        
        }
    }

    retainInclusiveRange(start, end) {
        wasm.icu4x_CodePointSetBuilder_retain_inclusive_range_mv1(this.ffiValue, diplomatRuntime.extractCodePoint(start, 'start'), diplomatRuntime.extractCodePoint(end, 'end'));
    
        try {
    
        } finally {
        
        }
    }

    retainSet(data) {
        wasm.icu4x_CodePointSetBuilder_retain_set_mv1(this.ffiValue, data.ffiValue);
    
        try {
    
        } finally {
        
        }
    }

    complementChar(ch) {
        wasm.icu4x_CodePointSetBuilder_complement_char_mv1(this.ffiValue, diplomatRuntime.extractCodePoint(ch, 'ch'));
    
        try {
    
        } finally {
        
        }
    }

    complementInclusiveRange(start, end) {
        wasm.icu4x_CodePointSetBuilder_complement_inclusive_range_mv1(this.ffiValue, diplomatRuntime.extractCodePoint(start, 'start'), diplomatRuntime.extractCodePoint(end, 'end'));
    
        try {
    
        } finally {
        
        }
    }

    complementSet(data) {
        wasm.icu4x_CodePointSetBuilder_complement_set_mv1(this.ffiValue, data.ffiValue);
    
        try {
    
        } finally {
        
        }
    }

    

}