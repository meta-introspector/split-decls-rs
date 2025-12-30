// Generated macro for impl_57 (impl)
macro_rules! Depcrate_processorimpl_57 {
() => {
// Module: crate::processor
// Provides: {"impl_57"}
// Dependencies: {}
impl < P , F > Processor for WithFallback < P , F > where P : Processor , F : Processor , { fn process (& self , tree : Tree) -> Result { self . primary . process (tree) . or_else (| err | { eprintln ! ("{err}, using fallback processor...") ; self . fallback . process (err . tree) }) } }
};
}
