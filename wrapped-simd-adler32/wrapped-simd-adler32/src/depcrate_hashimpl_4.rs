// Generated macro for impl_4 (impl)
macro_rules! Depcrate_hashimpl_4 {
() => {
// Module: crate::hash
// Provides: {"impl_4"}
// Dependencies: {}
impl Adler32Hash for & str { fn hash (& self) -> u32 { let mut hash = Adler32 :: new () ; hash . write (self . as_bytes ()) ; hash . finish () } }
};
}
