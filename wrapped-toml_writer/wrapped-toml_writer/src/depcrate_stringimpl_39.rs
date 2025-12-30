// Generated macro for impl_39 (impl)
macro_rules! Depcrate_stringimpl_39 {
() => {
// Module: crate::string
// Provides: {"impl_39"}
// Dependencies: {}
impl crate :: WriteTomlValue for TomlString < '_ > { fn write_toml_value < W : crate :: TomlWrite + ? Sized > (& self , writer : & mut W) -> core :: fmt :: Result { write_toml_value (self . decoded , Some (self . encoding) , self . newline , writer) } }
};
}
