// Generated macro for impl_70 (impl)
macro_rules! Depcrate_hygieneimpl_70 {
() => {
// Module: crate::hygiene
// Provides: {"impl_70"}
// Dependencies: {}
# [cfg (not (feature = "salsa"))] impl SyntaxContext { const MAX_ID : u32 = SALSA_MAX_ID_MIRROR - 1 ; pub const fn into_u32 (self) -> u32 { self . 0 } # [doc = " # Safety"] # [doc = ""] # [doc = " None. This is always safe to call without the `salsa` feature."] pub const unsafe fn from_u32 (u32 : u32) -> Self { Self (u32) } }
};
}
