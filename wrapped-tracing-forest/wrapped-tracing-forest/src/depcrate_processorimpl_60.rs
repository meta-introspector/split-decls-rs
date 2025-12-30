// Generated macro for impl_60 (impl)
macro_rules! Depcrate_processorimpl_60 {
() => {
// Module: crate::processor
// Provides: {"impl_60"}
// Dependencies: {}
impl < P : Processor > Processor for Box < P > { fn process (& self , tree : Tree) -> Result { self . as_ref () . process (tree) } }
};
}
