// Generated macro for impl_74 (impl)
macro_rules! Depcrate_valueimpl_74 {
() => {
// Module: crate::value
// Provides: {"impl_74"}
// Dependencies: {}
impl WriteTomlValue for str { fn write_toml_value < W : TomlWrite + ? Sized > (& self , writer : & mut W) -> core :: fmt :: Result { crate :: TomlStringBuilder :: new (self) . as_default () . write_toml_value (writer) } }
};
}
