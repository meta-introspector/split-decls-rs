// Generated macro for impl_26 (impl)
macro_rules! Depcrate_pointimpl_26 {
() => {
// Module: crate::point
// Provides: {"impl_26"}
// Dependencies: {}
# [cfg (feature = "zeroize")] impl < Size > Zeroize for EncodedPoint < Size > where Size : ModulusSize , { fn zeroize (& mut self) { self . bytes . zeroize () ; * self = Self :: identity () ; } }
};
}
