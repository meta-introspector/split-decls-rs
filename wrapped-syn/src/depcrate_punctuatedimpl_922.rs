// Generated macro for impl_922 (impl)
macro_rules! Depcrate_punctuatedimpl_922 {
() => {
// Module: crate::punctuated
// Provides: {"impl_922"}
// Dependencies: {}
impl < 'a , T , I > IterTrait < 'a , T > for I where T : 'a , I : DoubleEndedIterator < Item = & 'a T > + ExactSizeIterator < Item = & 'a T > + Clone + TrivialDrop + 'a , { fn clone_box (& self) -> Box < NoDrop < dyn IterTrait < 'a , T > + 'a > > { Box :: new (NoDrop :: new (self . clone ())) } }
};
}
