macro_rules! deps {
    () => {
        Adler32Imp!();
    };
}

macro_rules! get_imp_inner {
    () => {
        deps!();
        # [inline] # [cfg (not (all (target_feature = "simd128" , any (target_arch = "wasm32" , all (feature = "nightly" , target_arch = "wasm64")))))] fn get_imp_inner () -> Option < Adler32Imp > { None }
    };
}

get_imp_inner!();