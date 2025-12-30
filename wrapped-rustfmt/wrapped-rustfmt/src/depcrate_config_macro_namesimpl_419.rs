// Generated macro for impl_419 (impl)
macro_rules! Depcrate_config_macro_namesimpl_419 {
() => {
// Module: crate::config::macro_names
// Provides: {"impl_419"}
// Dependencies: {}
impl fmt :: Display for MacroSelector { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Self :: Name (name) => name . fmt (f) , Self :: All => write ! (f , "*") , } } }
};
}
