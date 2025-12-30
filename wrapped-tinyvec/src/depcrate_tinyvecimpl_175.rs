// Generated macro for impl_175 (impl)
macro_rules! Depcrate_tinyvecimpl_175 {
() => {
// Module: crate::tinyvec
// Provides: {"impl_175"}
// Dependencies: {}
impl < T , A > From < & '_ [T] > for TinyVec < A > where T : Clone + Default , A : Array < Item = T > , { # [inline] fn from (slice : & [T]) -> Self { if let Ok (arr) = ArrayVec :: try_from (slice) { TinyVec :: Inline (arr) } else { TinyVec :: Heap (slice . into ()) } } }
};
}
