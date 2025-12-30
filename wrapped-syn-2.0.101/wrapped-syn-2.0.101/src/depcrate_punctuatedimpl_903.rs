// Generated macro for impl_903 (impl)
macro_rules! Depcrate_punctuatedimpl_903 {
() => {
// Module: crate::punctuated
// Provides: {"impl_903"}
// Dependencies: {}
impl < 'a , T , P > ExactSizeIterator for PrivateIter < 'a , T , P > { fn len (& self) -> usize { self . inner . len () + self . last . len () } }
};
}
