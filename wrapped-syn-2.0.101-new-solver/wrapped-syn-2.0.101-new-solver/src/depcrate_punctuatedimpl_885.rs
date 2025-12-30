// Generated macro for impl_885 (impl)
macro_rules! Depcrate_punctuatedimpl_885 {
() => {
// Module: crate::punctuated
// Provides: {"impl_885"}
// Dependencies: {}
impl < T , P > ExactSizeIterator for IntoPairs < T , P > { fn len (& self) -> usize { self . inner . len () + self . last . len () } }
};
}
