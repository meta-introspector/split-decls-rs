// Generated macro for impl_180 (impl)
macro_rules! Depcrate_frameworkimpl_180 {
() => {
// Module: crate::framework
// Provides: {"impl_180"}
// Dependencies: {}
impl < T : Idx > GenKill < T > for DenseBitSet < T > { fn gen_ (& mut self , elem : T) { self . insert (elem) ; } fn kill (& mut self , elem : T) { self . remove (elem) ; } }
};
}
