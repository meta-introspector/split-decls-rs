// Generated macro for impl_80 (impl)
macro_rules! Depcrate_valueimpl_80 {
() => {
// Module: crate::value
// Provides: {"impl_80"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < K : WriteTomlKey , V : WriteTomlValue > WriteTomlValue for alloc :: collections :: BTreeMap < K , V > { fn write_toml_value < W : TomlWrite + ? Sized > (& self , writer : & mut W) -> core :: fmt :: Result { write_toml_inline_table (self . iter () , writer) } }
};
}
