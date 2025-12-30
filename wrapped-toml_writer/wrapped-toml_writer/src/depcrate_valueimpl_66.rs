// Generated macro for impl_66 (impl)
macro_rules! Depcrate_valueimpl_66 {
() => {
// Module: crate::value
// Provides: {"impl_66"}
// Dependencies: {}
impl WriteTomlValue for i32 { fn write_toml_value < W : TomlWrite + ? Sized > (& self , writer : & mut W) -> core :: fmt :: Result { write ! (writer , "{self}") } }
};
}
