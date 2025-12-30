// Generated macro for impl_3 (impl)
macro_rules! Depcrate_hashimpl_3 {
() => {
// Module: crate::hash
// Provides: {"impl_3"}
// Dependencies: {}
impl Adler32Hash for & [u8] { fn hash (& self) -> u32 { let mut hash = Adler32 :: new () ; hash . write (self) ; hash . finish () } }
};
}
