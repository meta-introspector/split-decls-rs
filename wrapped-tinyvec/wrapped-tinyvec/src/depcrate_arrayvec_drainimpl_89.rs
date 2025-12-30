// Generated macro for impl_89 (impl)
macro_rules! Depcrate_arrayvec_drainimpl_89 {
() => {
// Module: crate::arrayvec_drain
// Provides: {"impl_89"}
// Dependencies: {}
impl < 'a , T : 'a + Default > DoubleEndedIterator for ArrayVecDrain < 'a , T > { # [inline] fn next_back (& mut self) -> Option < Self :: Item > { self . iter . next_back () . map (core :: mem :: take) } # [inline] fn nth_back (& mut self , n : usize) -> Option < Self :: Item > { self . iter . nth_back (n) . map (core :: mem :: take) } }
};
}
