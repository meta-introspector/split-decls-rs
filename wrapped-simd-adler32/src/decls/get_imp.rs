macro_rules! deps {
    () => {
        Adler32Imp!();
    };
}

macro_rules! get_imp {
    () => {
        deps!();
        pub fn get_imp () -> Adler32Imp { avx512 :: get_imp () . or_else (avx2 :: get_imp) . or_else (ssse3 :: get_imp) . or_else (sse2 :: get_imp) . or_else (wasm :: get_imp) . unwrap_or (scalar :: update) }
    };
}

get_imp!();