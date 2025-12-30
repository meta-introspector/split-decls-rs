// Generated macro for impl_90 (impl)
macro_rules! Depcrate_arrayvec_drainimpl_90 {
() => {
// Module: crate::arrayvec_drain
// Provides: {"impl_90"}
// Dependencies: {}
impl < 'a , T : 'a + Default > Iterator for ArrayVecDrain < 'a , T > { type Item = T ; # [inline] fn next (& mut self) -> Option < Self :: Item > { self . iter . next () . map (core :: mem :: take) } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } # [inline] fn nth (& mut self , n : usize) -> Option < Self :: Item > { self . iter . nth (n) . map (core :: mem :: take) } # [inline] fn last (self) -> Option < Self :: Item > { self . iter . last () . map (core :: mem :: take) } # [inline] fn for_each < F > (self , f : F) where F : FnMut (Self :: Item) , { self . iter . map (core :: mem :: take) . for_each (f) } }
};
}
