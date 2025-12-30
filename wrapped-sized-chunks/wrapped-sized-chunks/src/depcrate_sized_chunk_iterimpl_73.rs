// Generated macro for impl_73 (impl)
macro_rules! Depcrate_sized_chunk_iterimpl_73 {
() => {
// Module: crate::sized_chunk::iter
// Provides: {"impl_73"}
// Dependencies: {}
impl < 'a , A , const N : usize > Iterator for Drain < 'a , A , N > where A : 'a , { type Item = A ; fn next (& mut self) -> Option < Self :: Item > { if self . chunk . is_empty () { None } else { Some (self . chunk . pop_front ()) } } fn size_hint (& self) -> (usize , Option < usize >) { (self . chunk . len () , Some (self . chunk . len ())) } }
};
}
