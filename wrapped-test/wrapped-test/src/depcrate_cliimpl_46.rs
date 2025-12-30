// Generated macro for impl_46 (impl)
macro_rules! Depcrate_cliimpl_46 {
() => {
// Module: crate::cli
// Provides: {"impl_46"}
// Dependencies: {}
impl TestOpts { pub fn use_color (& self) -> bool { match self . color { ColorConfig :: AutoColor => ! self . nocapture && io :: stdout () . is_terminal () , ColorConfig :: AlwaysColor => true , ColorConfig :: NeverColor => false , } } }
};
}
