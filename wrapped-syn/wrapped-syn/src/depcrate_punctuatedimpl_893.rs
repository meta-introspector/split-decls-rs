// Generated macro for impl_893 (impl)
macro_rules! Depcrate_punctuatedimpl_893 {
() => {
// Module: crate::punctuated
// Provides: {"impl_893"}
// Dependencies: {}
impl < 'a , T , P > ExactSizeIterator for Pairs < 'a , T , P > { fn len (& self) -> usize { self . inner . len () + self . last . len () } }
};
}
