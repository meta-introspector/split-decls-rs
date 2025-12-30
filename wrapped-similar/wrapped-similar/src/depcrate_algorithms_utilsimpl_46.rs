// Generated macro for impl_46 (impl)
macro_rules! Depcrate_algorithms_utilsimpl_46 {
() => {
// Module: crate::algorithms::utils
// Provides: {"impl_46"}
// Dependencies: {}
impl < Idx : ? Sized > UniqueItem < '_ , Idx > where Idx : Index < usize > , { # [doc = " Returns the value."] # [inline (always)] pub fn value (& self) -> & Idx :: Output { & self . lookup [self . index] } # [doc = " Returns the original index."] # [inline (always)] pub fn original_index (& self) -> usize { self . index } }
};
}
