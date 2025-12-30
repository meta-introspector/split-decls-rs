// Generated macro for impl_6 (impl)
macro_rules! Depcrateimpl_6 {
() => {
// Module: crate
// Provides: {"impl_6"}
// Dependencies: {}
# [cfg (all (feature = "sha2" , not (any (target_os = "solana" , target_arch = "bpf"))))] impl Hasher { # [inline (always)] pub fn hash (& mut self , val : & [u8]) { self . hasher . update (val) ; } # [inline (always)] pub fn hashv (& mut self , vals : & [& [u8]]) { for val in vals { self . hash (val) ; } } # [inline (always)] pub fn result (self) -> Hash { let bytes : [u8 ; HASH_BYTES] = self . hasher . finalize () . into () ; bytes . into () } }
};
}
