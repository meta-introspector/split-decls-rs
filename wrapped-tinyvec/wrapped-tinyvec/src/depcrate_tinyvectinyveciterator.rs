// Generated macro for TinyVecIterator (enum)
macro_rules! Depcrate_tinyvecTinyVecIterator {
() => {
// Module: crate::tinyvec
// Provides: {"TinyVecIterator"}
// Dependencies: {}
# [doc = " Iterator for consuming an `TinyVec` and returning owned elements."] # [cfg_attr (docsrs , doc (cfg (feature = "alloc")))] pub enum TinyVecIterator < A : Array > { # [allow (missing_docs)] Inline (ArrayVecIterator < A >) , # [allow (missing_docs)] Heap (alloc :: vec :: IntoIter < A :: Item >) , }
};
}
