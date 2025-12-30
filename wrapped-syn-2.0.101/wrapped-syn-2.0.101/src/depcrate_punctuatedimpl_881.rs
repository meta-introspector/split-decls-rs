// Generated macro for impl_881 (impl)
macro_rules! Depcrate_punctuatedimpl_881 {
() => {
// Module: crate::punctuated
// Provides: {"impl_881"}
// Dependencies: {}
impl < 'a , T , P > ExactSizeIterator for PairsMut < 'a , T , P > { fn len (& self) -> usize { self . inner . len () + self . last . len () } }
};
}
