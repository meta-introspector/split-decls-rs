// Generated macro for impl_141 (impl)
macro_rules! Depcrate_error_formatimpl_141 {
() => {
// Module: crate::error::format
// Provides: {"impl_141"}
// Dependencies: {}
# [cfg (feature = "serde")] impl Format { # [doc = " Obtain an error type for the serializer."] # [doc (hidden)] # [inline] pub fn into_invalid_serde_value < S : serde_core :: Serializer > (self) -> S :: Error { use serde_core :: ser :: Error ; S :: Error :: custom (self) } }
};
}
