// Generated macro for impl_160 (impl)
macro_rules! Depcrate_tinyvecimpl_160 {
() => {
// Module: crate::tinyvec
// Provides: {"impl_160"}
// Dependencies: {}
impl < 'p , A : Array > Iterator for TinyVecDrain < 'p , A > { type Item = A :: Item ; impl_mirrored ! { type Mirror = TinyVecDrain ; # [inline] fn next (self : & mut Self) -> Option < Self :: Item >; # [inline] fn nth (self : & mut Self , n : usize) -> Option < Self :: Item >; # [inline] fn size_hint (self : & Self) -> (usize , Option < usize >) ; # [inline] fn last (self : Self) -> Option < Self :: Item >; # [inline] fn count (self : Self) -> usize ; } # [inline] fn for_each < F : FnMut (Self :: Item) > (self , f : F) { match self { TinyVecDrain :: Inline (i) => i . for_each (f) , TinyVecDrain :: Heap (h) => h . for_each (f) , } } }
};
}
