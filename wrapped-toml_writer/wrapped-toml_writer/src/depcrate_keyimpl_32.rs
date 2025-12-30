// Generated macro for impl_32 (impl)
macro_rules! Depcrate_keyimpl_32 {
() => {
// Module: crate::key
// Provides: {"impl_32"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl WriteTomlKey for String { fn write_toml_key < W : TomlWrite + ? Sized > (& self , writer : & mut W) -> core :: fmt :: Result { self . as_str () . write_toml_key (writer) } }
};
}
