// Generated macro for get_imp (function)
macro_rules! Depcrate_impget_imp {
() => {
// Module: crate::imp
// Provides: {"get_imp"}
// Dependencies: {}
pub fn get_imp () -> Adler32Imp { avx512 :: get_imp () . or_else (avx2 :: get_imp) . or_else (ssse3 :: get_imp) . or_else (sse2 :: get_imp) . or_else (wasm :: get_imp) . unwrap_or (scalar :: update) }
};
}
