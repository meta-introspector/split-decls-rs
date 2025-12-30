// Generated macro for impl_63 (impl)
macro_rules! Depcrate_valueimpl_63 {
() => {
// Module: crate::value
// Provides: {"impl_63"}
// Dependencies: {}
impl WriteTomlValue for u16 { fn write_toml_value < W : TomlWrite + ? Sized > (& self , writer : & mut W) -> core :: fmt :: Result { write ! (writer , "{self}") } }
};
}
