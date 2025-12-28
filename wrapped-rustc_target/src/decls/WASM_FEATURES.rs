macro_rules! deps {
    () => {
        Stability!();
        ImpliedFeatures!();
    };
}

macro_rules! WASM_FEATURES {
    () => {
        deps!();
        static WASM_FEATURES : & [(& str , Stability , ImpliedFeatures)] = & [("atomics" , Unstable (sym :: wasm_target_feature) , & []) , ("bulk-memory" , Stable , & []) , ("exception-handling" , Unstable (sym :: wasm_target_feature) , & []) , ("extended-const" , Stable , & []) , ("multivalue" , Stable , & []) , ("mutable-globals" , Stable , & []) , ("nontrapping-fptoint" , Stable , & []) , ("reference-types" , Stable , & []) , ("relaxed-simd" , Stable , & ["simd128"]) , ("sign-ext" , Stable , & []) , ("simd128" , Stable , & []) , ("tail-call" , Stable , & []) , ("wide-arithmetic" , Unstable (sym :: wasm_target_feature) , & []) ,] ;
    };
}

WASM_FEATURES!()