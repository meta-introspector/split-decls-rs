// Generated macro for impl_84 (impl)
macro_rules! Depcrate_libs_simd_jsonimpl_84 {
() => {
// Module: crate::libs::simd_json
// Provides: {"impl_84"}
// Dependencies: {}
impl TypeSize for simd_json :: OwnedValue { fn extra_size (& self) -> usize { match self { simd_json :: OwnedValue :: Static (value) => value . extra_size () , simd_json :: OwnedValue :: String (value) => value . extra_size () , simd_json :: OwnedValue :: Object (value) => value . extra_size () , simd_json :: OwnedValue :: Array (value) => value . extra_size () , } } }
};
}
