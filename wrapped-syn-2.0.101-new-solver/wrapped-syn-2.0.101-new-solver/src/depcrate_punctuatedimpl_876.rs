// Generated macro for impl_876 (impl)
macro_rules! Depcrate_punctuatedimpl_876 {
() => {
// Module: crate::punctuated
// Provides: {"impl_876"}
// Dependencies: {}
impl < 'a , T , P > ExactSizeIterator for Pairs < 'a , T , P > { fn len (& self) -> usize { self . inner . len () + self . last . len () } }
};
}
