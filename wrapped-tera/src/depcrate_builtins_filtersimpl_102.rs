// Generated macro for impl_102 (impl)
macro_rules! Depcrate_builtins_filtersimpl_102 {
() => {
// Module: crate::builtins::filters
// Provides: {"impl_102"}
// Dependencies: {}
impl < F > Filter for F where F : Fn (& Value , & HashMap < String , Value >) -> Result < Value > + Sync + Send , { fn filter (& self , value : & Value , args : & HashMap < String , Value >) -> Result < Value > { self (value , args) } }
};
}
