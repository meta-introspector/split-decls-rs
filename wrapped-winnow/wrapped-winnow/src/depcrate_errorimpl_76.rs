// Generated macro for impl_76 (impl)
macro_rules! Depcrate_errorimpl_76 {
() => {
// Module: crate::error
// Provides: {"impl_76"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < I : ToOwned > InputError < & I > where < I as ToOwned > :: Owned : Clone , { # [doc = " Obtaining ownership"] pub fn into_owned (self) -> InputError < < I as ToOwned > :: Owned > { self . map_input (ToOwned :: to_owned) } }
};
}
