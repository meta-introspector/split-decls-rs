// Generated macro for impl_176 (impl)
macro_rules! Depcrate_tinyvecimpl_176 {
() => {
// Module: crate::tinyvec
// Provides: {"impl_176"}
// Dependencies: {}
impl < T , A > From < & '_ mut [T] > for TinyVec < A > where T : Clone + Default , A : Array < Item = T > , { # [inline] fn from (slice : & mut [T]) -> Self { Self :: from (& * slice) } }
};
}
