// Generated macro for impl_106 (impl)
macro_rules! Depcrate_error_component_rangeimpl_106 {
() => {
// Module: crate::error::component_range
// Provides: {"impl_106"}
// Dependencies: {}
impl fmt :: Display for ComponentRange { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "{} must be in the range {}..={}" , self . name , self . minimum , self . maximum) ? ; if let Some (message) = self . conditional_message { write ! (f , " {message}") ? ; } Ok (()) } }
};
}
