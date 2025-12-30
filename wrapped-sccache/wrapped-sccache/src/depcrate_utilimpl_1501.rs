// Generated macro for impl_1501 (impl)
macro_rules! Depcrate_utilimpl_1501 {
() => {
// Module: crate::util
// Provides: {"impl_1501"}
// Dependencies: {}
impl Hasher for HashToDigest < '_ > { fn write (& mut self , bytes : & [u8]) { self . digest . update (bytes) ; } fn finish (& self) -> u64 { panic ! ("not supposed to be called") ; } }
};
}
