// Generated macro for impl_441 (impl)
macro_rules! Depcrate_varzerovec_databakeimpl_441 {
() => {
// Module: crate::varzerovec::databake
// Provides: {"impl_441"}
// Dependencies: {}
impl < T : VarULE + ? Sized > BakeSize for & VarZeroSlice < T , Index16 > { fn borrows_size (& self) -> usize { if self . is_empty () { 0 } else { self . as_bytes () . len () } } }
};
}
