// Generated macro for impl_33 (impl)
macro_rules! Depcrate_keyimpl_33 {
() => {
// Module: crate::key
// Provides: {"impl_33"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl WriteTomlKey for Cow < '_ , str > { fn write_toml_key < W : TomlWrite + ? Sized > (& self , writer : & mut W) -> core :: fmt :: Result { self . as_ref () . write_toml_key (writer) } }
};
}
