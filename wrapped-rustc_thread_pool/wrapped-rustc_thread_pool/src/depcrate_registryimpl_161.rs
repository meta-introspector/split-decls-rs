// Generated macro for impl_161 (impl)
macro_rules! Depcrate_registryimpl_161 {
() => {
// Module: crate::registry
// Provides: {"impl_161"}
// Dependencies: {}
impl < F > ThreadSpawn for CustomSpawn < F > where F : FnMut (ThreadBuilder) -> io :: Result < () > , { private_impl ! { } # [inline] fn spawn (& mut self , thread : ThreadBuilder) -> io :: Result < () > { (self . 0) (thread) } }
};
}
