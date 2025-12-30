// Generated macro for impl_81 (impl)
macro_rules! Depcrate_valueimpl_81 {
() => {
// Module: crate::value
// Provides: {"impl_81"}
// Dependencies: {}
# [cfg (feature = "std")] impl < K : WriteTomlKey , V : WriteTomlValue > WriteTomlValue for std :: collections :: HashMap < K , V > { fn write_toml_value < W : TomlWrite + ? Sized > (& self , writer : & mut W) -> core :: fmt :: Result { write_toml_inline_table (self . iter () , writer) } }
};
}
