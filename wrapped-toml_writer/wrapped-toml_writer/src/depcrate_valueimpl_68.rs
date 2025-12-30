// Generated macro for impl_68 (impl)
macro_rules! Depcrate_valueimpl_68 {
() => {
// Module: crate::value
// Provides: {"impl_68"}
// Dependencies: {}
impl WriteTomlValue for i64 { fn write_toml_value < W : TomlWrite + ? Sized > (& self , writer : & mut W) -> core :: fmt :: Result { write ! (writer , "{self}") } }
};
}
