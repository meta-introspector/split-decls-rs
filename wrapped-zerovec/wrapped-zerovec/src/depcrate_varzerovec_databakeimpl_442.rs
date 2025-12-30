// Generated macro for impl_442 (impl)
macro_rules! Depcrate_varzerovec_databakeimpl_442 {
() => {
// Module: crate::varzerovec::databake
// Provides: {"impl_442"}
// Dependencies: {}
impl < T : VarULE + ? Sized > BakeSize for & VarZeroSlice < T , Index32 > { fn borrows_size (& self) -> usize { if self . is_empty () { 0 } else { self . as_bytes () . len () } } }
};
}
