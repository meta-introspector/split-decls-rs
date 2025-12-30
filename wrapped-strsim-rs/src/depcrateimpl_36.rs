// Generated macro for impl_36 (impl)
macro_rules! Depcrateimpl_36 {
() => {
// Module: crate
// Provides: {"impl_36"}
// Dependencies: {}
impl < ValueType > Default for HybridGrowingHashmapChar < ValueType > where ValueType : Default + Clone + Copy + Eq , { fn default () -> Self { HybridGrowingHashmapChar { map : GrowingHashmapChar :: default () , extended_ascii : [Default :: default () ; 256] , } } }
};
}
