macro_rules! WASM_FEATURES_FOR_CORRECT_VECTOR_ABI {
    () => {
        const WASM_FEATURES_FOR_CORRECT_VECTOR_ABI : & 'static [(u64 , & 'static str)] = & [(128 , "simd128")] ;
    };
}

WASM_FEATURES_FOR_CORRECT_VECTOR_ABI!();