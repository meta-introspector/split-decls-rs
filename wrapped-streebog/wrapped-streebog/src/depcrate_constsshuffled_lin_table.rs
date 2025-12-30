// Generated macro for SHUFFLED_LIN_TABLE (const)
macro_rules! Depcrate_constsSHUFFLED_LIN_TABLE {
() => {
// Module: crate::consts
// Provides: {"SHUFFLED_LIN_TABLE"}
// Dependencies: {}
# [doc = " Precomputed, pre-shuffled table for linear transformation using matrix"] # [doc = " `const::A` and shuffled using `const::P`"] pub (crate) const SHUFFLED_LIN_TABLE : [[u64 ; 256] ; 8] = { let mut table = [[0u64 ; 256] ; 8] ; let mut i = 0 ; while i < 8 { let mut j = 0 ; while j < 256 { let mut accum = 0u64 ; let mut k = 0 ; while k < 8 { if P [j] & (1u8 << k) != 0 { accum ^= A [8 * i + k] ; } k += 1 ; } table [i] [j] = accum ; j += 1 ; } i += 1 ; } table } ;
};
}
