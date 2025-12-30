// Generated macro for impl_13 (impl)
macro_rules! Depcrateimpl_13 {
() => {
// Module: crate
// Provides: {"impl_13"}
// Dependencies: {}
impl FromIterator < (u64 , Hash) > for SlotHashes { fn from_iter < I : IntoIterator < Item = (u64 , Hash) > > (iter : I) -> Self { Self (iter . into_iter () . collect ()) } }
};
}
