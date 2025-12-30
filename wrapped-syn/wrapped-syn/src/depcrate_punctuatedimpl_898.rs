// Generated macro for impl_898 (impl)
macro_rules! Depcrate_punctuatedimpl_898 {
() => {
// Module: crate::punctuated
// Provides: {"impl_898"}
// Dependencies: {}
impl < 'a , T , P > ExactSizeIterator for PairsMut < 'a , T , P > { fn len (& self) -> usize { self . inner . len () + self . last . len () } }
};
}
