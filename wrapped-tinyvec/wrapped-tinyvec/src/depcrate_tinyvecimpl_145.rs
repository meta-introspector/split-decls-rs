// Generated macro for impl_145 (impl)
macro_rules! Depcrate_tinyvecimpl_145 {
() => {
// Module: crate::tinyvec
// Provides: {"impl_145"}
// Dependencies: {}
impl < A > Clone for TinyVec < A > where A : Array + Clone , A :: Item : Clone , { # [inline] fn clone (& self) -> Self { match self { TinyVec :: Heap (v) => TinyVec :: Heap (v . clone ()) , TinyVec :: Inline (v) => TinyVec :: Inline (v . clone ()) , } } # [inline] fn clone_from (& mut self , o : & Self) { if o . len () > self . len () { self . reserve (o . len () - self . len ()) ; } else { self . truncate (o . len ()) ; } let (start , end) = o . split_at (self . len ()) ; for (dst , src) in self . iter_mut () . zip (start) { dst . clone_from (src) ; } self . extend_from_slice (end) ; } }
};
}
