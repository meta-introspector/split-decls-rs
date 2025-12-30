// Generated macro for impl_43 (impl)
macro_rules! Depcrate_stringimpl_43 {
() => {
// Module: crate::string
// Provides: {"impl_43"}
// Dependencies: {}
impl crate :: WriteTomlKey for TomlKey < '_ > { fn write_toml_key < W : crate :: TomlWrite + ? Sized > (& self , writer : & mut W) -> core :: fmt :: Result { let newline = false ; write_toml_value (self . decoded , self . encoding , newline , writer) } }
};
}
