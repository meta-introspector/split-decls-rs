// Generated macro for impl_110 (impl)
macro_rules! Depcrate_error_component_rangeimpl_110 {
() => {
// Module: crate::error::component_range
// Provides: {"impl_110"}
// Dependencies: {}
# [cfg (feature = "serde")] impl ComponentRange { # [doc = " Convert the error to a deserialization error."] # [inline] pub (crate) fn into_de_error < E : serde_core :: de :: Error > (self) -> E { E :: invalid_value (serde_core :: de :: Unexpected :: Signed (self . value) , & self) } }
};
}
