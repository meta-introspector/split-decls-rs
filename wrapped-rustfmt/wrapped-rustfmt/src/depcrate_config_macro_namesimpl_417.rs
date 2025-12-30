// Generated macro for impl_417 (impl)
macro_rules! Depcrate_config_macro_namesimpl_417 {
() => {
// Module: crate::config::macro_names
// Provides: {"impl_417"}
// Dependencies: {}
impl fmt :: Display for MacroSelector { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Self :: Name (name) => name . fmt (f) , Self :: All => write ! (f , "*") , } } }
};
}
