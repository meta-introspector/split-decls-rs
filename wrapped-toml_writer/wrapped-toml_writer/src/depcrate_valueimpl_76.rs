// Generated macro for impl_76 (impl)
macro_rules! Depcrate_valueimpl_76 {
() => {
// Module: crate::value
// Provides: {"impl_76"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl WriteTomlValue for Cow < '_ , str > { fn write_toml_value < W : TomlWrite + ? Sized > (& self , writer : & mut W) -> core :: fmt :: Result { self . as_ref () . write_toml_value (writer) } }
};
}
