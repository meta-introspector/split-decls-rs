// Generated macro for impl_59 (impl)
macro_rules! Depcrate_processorimpl_59 {
() => {
// Module: crate::processor
// Provides: {"impl_59"}
// Dependencies: {}
impl < F > Processor for FromFn < F > where F : 'static + Fn (Tree) -> Result , { fn process (& self , tree : Tree) -> Result { (self . 0) (tree) } }
};
}
