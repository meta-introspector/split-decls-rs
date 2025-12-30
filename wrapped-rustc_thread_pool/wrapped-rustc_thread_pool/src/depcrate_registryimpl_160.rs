// Generated macro for impl_160 (impl)
macro_rules! Depcrate_registryimpl_160 {
() => {
// Module: crate::registry
// Provides: {"impl_160"}
// Dependencies: {}
impl < F > CustomSpawn < F > where F : FnMut (ThreadBuilder) -> io :: Result < () > , { pub (super) fn new (spawn : F) -> Self { CustomSpawn (spawn) } }
};
}
