// Generated macro for impl_902 (impl)
macro_rules! Depcrate_punctuatedimpl_902 {
() => {
// Module: crate::punctuated
// Provides: {"impl_902"}
// Dependencies: {}
impl < T , P > ExactSizeIterator for IntoPairs < T , P > { fn len (& self) -> usize { self . inner . len () + self . last . len () } }
};
}
