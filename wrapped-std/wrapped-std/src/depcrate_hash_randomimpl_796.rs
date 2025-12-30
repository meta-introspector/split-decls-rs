// Generated macro for impl_796 (impl)
macro_rules! Depcrate_hash_randomimpl_796 {
() => {
// Module: crate::hash::random
// Provides: {"impl_796"}
// Dependencies: {}
# [stable (feature = "hashmap_default_hasher" , since = "1.13.0")] impl Hasher for DefaultHasher { # [inline] fn write (& mut self , msg : & [u8]) { self . 0 . write (msg) } # [inline] fn write_str (& mut self , s : & str) { self . 0 . write_str (s) ; } # [inline] fn finish (& self) -> u64 { self . 0 . finish () } }
};
}
