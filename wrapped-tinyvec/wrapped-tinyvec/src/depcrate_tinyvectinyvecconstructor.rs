// Generated macro for TinyVecConstructor (enum)
macro_rules! Depcrate_tinyvecTinyVecConstructor {
() => {
// Module: crate::tinyvec
// Provides: {"TinyVecConstructor"}
// Dependencies: {}
# [doc (hidden)] pub enum TinyVecConstructor < A : Array > { Inline (fn (ArrayVec < A >) -> TinyVec < A >) , Heap (fn (Vec < A :: Item >) -> TinyVec < A >) , }
};
}
