// Generated macro for impl_121 (impl)
macro_rules! Depcrate_xxhash3_64impl_121 {
() => {
// Module: crate::xxhash3_64
// Provides: {"impl_121"}
// Dependencies: {}
impl < S > hash :: Hasher for RawHasher < S > where S : FixedBuffer , { # [inline] fn write (& mut self , input : & [u8]) { self . 0 . write (input) ; } # [inline] fn finish (& self) -> u64 { self . 0 . finish (Finalize64) } }
};
}
