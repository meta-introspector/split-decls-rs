// Generated macro for impl_109 (impl)
macro_rules! Depcrate_error_component_rangeimpl_109 {
() => {
// Module: crate::error::component_range
// Provides: {"impl_109"}
// Dependencies: {}
# [doc = " **This trait implementation is deprecated and will be removed in a future breaking release.**"] # [cfg (feature = "serde")] impl serde_core :: de :: Expected for ComponentRange { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "a value in the range {}..={}" , self . minimum , self . maximum) } }
};
}
