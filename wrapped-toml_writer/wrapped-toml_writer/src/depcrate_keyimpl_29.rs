// Generated macro for impl_29 (impl)
macro_rules! Depcrate_keyimpl_29 {
() => {
// Module: crate::key
// Provides: {"impl_29"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < T > ToTomlKey for T where T : WriteTomlKey + ? Sized , { fn to_toml_key (& self) -> String { let mut result = String :: new () ; let _ = self . write_toml_key (& mut result) ; result } }
};
}
