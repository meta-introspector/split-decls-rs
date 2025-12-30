// Generated macro for impl_75 (impl)
macro_rules! Depcrate_valueimpl_75 {
() => {
// Module: crate::value
// Provides: {"impl_75"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl WriteTomlValue for String { fn write_toml_value < W : TomlWrite + ? Sized > (& self , writer : & mut W) -> core :: fmt :: Result { self . as_str () . write_toml_value (writer) } }
};
}
