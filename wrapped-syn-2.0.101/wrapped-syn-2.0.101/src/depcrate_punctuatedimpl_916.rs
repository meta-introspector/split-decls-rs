// Generated macro for impl_916 (impl)
macro_rules! Depcrate_punctuatedimpl_916 {
() => {
// Module: crate::punctuated
// Provides: {"impl_916"}
// Dependencies: {}
impl < 'a , T , P > ExactSizeIterator for PrivateIterMut < 'a , T , P > { fn len (& self) -> usize { self . inner . len () + self . last . len () } }
};
}
