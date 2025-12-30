// Generated macro for impl_110 (impl)
macro_rules! Depcrate_builtins_functionsimpl_110 {
() => {
// Module: crate::builtins::functions
// Provides: {"impl_110"}
// Dependencies: {}
impl < F > Function for F where F : Fn (& HashMap < String , Value >) -> Result < Value > + Sync + Send , { fn call (& self , args : & HashMap < String , Value >) -> Result < Value > { self (args) } }
};
}
