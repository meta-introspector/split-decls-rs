// Generated macro for impl_123 (impl)
macro_rules! Depcrate_builtins_testersimpl_123 {
() => {
// Module: crate::builtins::testers
// Provides: {"impl_123"}
// Dependencies: {}
impl < F > Test for F where F : Fn (Option < & Value > , & [Value]) -> Result < bool > + Sync + Send , { fn test (& self , value : Option < & Value > , args : & [Value]) -> Result < bool > { self (value , args) } }
};
}
