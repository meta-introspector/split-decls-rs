// Generated macro for impl_62 (impl)
macro_rules! Depcrate_valueimpl_62 {
() => {
// Module: crate::value
// Provides: {"impl_62"}
// Dependencies: {}
impl WriteTomlValue for i8 { fn write_toml_value < W : TomlWrite + ? Sized > (& self , writer : & mut W) -> core :: fmt :: Result { write ! (writer , "{self}") } }
};
}
