// Generated macro for impl_119 (impl)
macro_rules! Depcrate_errorimpl_119 {
() => {
// Module: crate::error
// Provides: {"impl_119"}
// Dependencies: {}
# [cfg (feature = "std")] impl < I : ToOwned , C > TreeError < & I , C > { # [doc = " Obtaining ownership"] pub fn into_owned (self) -> TreeError < < I as ToOwned > :: Owned , C > { self . map_input (ToOwned :: to_owned) } }
};
}
