// Generated macro for adler32 (function)
macro_rules! Depcrate_adler32adler32 {
() => {
// Module: crate::adler32
// Provides: {"adler32"}
// Dependencies: {}
pub fn adler32 (start_checksum : u32 , data : & [u8]) -> u32 { # [cfg (target_arch = "x86_64")] if crate :: cpu_features :: is_enabled_avx2_and_bmi2 () { return avx2 :: adler32_avx2 (start_checksum , data) ; } # [cfg (target_arch = "aarch64")] if crate :: cpu_features :: is_enabled_neon () { return self :: neon :: adler32_neon (start_checksum , data) ; } # [cfg (any (target_arch = "wasm32" , target_arch = "wasm64"))] if crate :: cpu_features :: is_enabled_simd128 () { return self :: wasm :: adler32_wasm (start_checksum , data) ; } generic :: adler32_rust (start_checksum , data) }
};
}
