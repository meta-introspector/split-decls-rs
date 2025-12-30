// Generated macro for impl_920 (impl)
macro_rules! Depcrate_punctuatedimpl_920 {
() => {
// Module: crate::punctuated
// Provides: {"impl_920"}
// Dependencies: {}
impl < 'a , T , P > ExactSizeIterator for PrivateIter < 'a , T , P > { fn len (& self) -> usize { self . inner . len () + self . last . len () } }
};
}
