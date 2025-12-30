// Generated macro for impl_5 (impl)
macro_rules! Depcrate_hashimpl_5 {
() => {
// Module: crate::hash
// Provides: {"impl_5"}
// Dependencies: {}
# [cfg (feature = "const-generics")] impl < const SIZE : usize > Adler32Hash for [u8 ; SIZE] { fn hash (& self) -> u32 { let mut hash = Adler32 :: new () ; hash . write (self) ; hash . finish () } }
};
}
