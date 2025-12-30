// Generated macro for impl_143 (impl)
macro_rules! Depcrate_xxhash3_128impl_143 {
() => {
// Module: crate::xxhash3_128
// Provides: {"impl_143"}
// Dependencies: {}
impl < S > RawHasher < S > where S : FixedBuffer , { # [doc = " Writes some data into this `Hasher`."] # [inline] pub fn write (& mut self , input : & [u8]) { self . 0 . write (input) ; } # [doc = " Returns the hash value for the values written so"] # [doc = " far. Unlike [`std::hash::Hasher::finish`][], this method"] # [doc = " returns the complete 128-bit value calculated, not a"] # [doc = " 64-bit value."] # [inline] pub fn finish_128 (& self) -> u128 { self . 0 . finish (Finalize128) } }
};
}
