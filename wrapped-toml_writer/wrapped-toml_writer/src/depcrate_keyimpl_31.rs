// Generated macro for impl_31 (impl)
macro_rules! Depcrate_keyimpl_31 {
() => {
// Module: crate::key
// Provides: {"impl_31"}
// Dependencies: {}
impl WriteTomlKey for str { fn write_toml_key < W : TomlWrite + ? Sized > (& self , writer : & mut W) -> core :: fmt :: Result { crate :: TomlKeyBuilder :: new (self) . as_default () . write_toml_key (writer) } }
};
}
