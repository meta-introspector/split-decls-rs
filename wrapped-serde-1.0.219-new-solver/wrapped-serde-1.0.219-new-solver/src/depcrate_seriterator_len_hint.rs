// Generated macro for iterator_len_hint (function)
macro_rules! Depcrate_seriterator_len_hint {
() => {
// Module: crate::ser
// Provides: {"iterator_len_hint"}
// Dependencies: {}
fn iterator_len_hint < I > (iter : & I) -> Option < usize > where I : Iterator , { match iter . size_hint () { (lo , Some (hi)) if lo == hi => Some (lo) , _ => None , } }
};
}
