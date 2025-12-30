// Generated macro for impl_479 (impl)
macro_rules! Depcrate_zerovec_databakeimpl_479 {
() => {
// Module: crate::zerovec::databake
// Provides: {"impl_479"}
// Dependencies: {}
impl < T : AsULE > BakeSize for ZeroVec < '_ , T > { fn borrows_size (& self) -> usize { self . as_bytes () . len () } }
};
}
