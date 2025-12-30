// Generated macro for impl_933 (impl)
macro_rules! Depcrate_punctuatedimpl_933 {
() => {
// Module: crate::punctuated
// Provides: {"impl_933"}
// Dependencies: {}
impl < 'a , T , P > ExactSizeIterator for PrivateIterMut < 'a , T , P > { fn len (& self) -> usize { self . inner . len () + self . last . len () } }
};
}
