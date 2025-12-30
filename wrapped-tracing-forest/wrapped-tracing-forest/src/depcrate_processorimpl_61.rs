// Generated macro for impl_61 (impl)
macro_rules! Depcrate_processorimpl_61 {
() => {
// Module: crate::processor
// Provides: {"impl_61"}
// Dependencies: {}
impl < P : Processor > Processor for Arc < P > { fn process (& self , tree : Tree) -> Result { self . as_ref () . process (tree) } }
};
}
