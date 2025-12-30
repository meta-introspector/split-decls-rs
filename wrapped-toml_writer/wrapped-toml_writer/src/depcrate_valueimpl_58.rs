// Generated macro for impl_58 (impl)
macro_rules! Depcrate_valueimpl_58 {
() => {
// Module: crate::value
// Provides: {"impl_58"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < T > ToTomlValue for T where T : WriteTomlValue + ? Sized , { fn to_toml_value (& self) -> String { let mut result = String :: new () ; let _ = self . write_toml_value (& mut result) ; result } }
};
}
