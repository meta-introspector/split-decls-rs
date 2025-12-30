// Generated macro for impl_79 (impl)
macro_rules! Depcrate_valueimpl_79 {
() => {
// Module: crate::value
// Provides: {"impl_79"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < V : WriteTomlValue > WriteTomlValue for Vec < V > { fn write_toml_value < W : TomlWrite + ? Sized > (& self , writer : & mut W) -> core :: fmt :: Result { self . as_slice () . write_toml_value (writer) } }
};
}
