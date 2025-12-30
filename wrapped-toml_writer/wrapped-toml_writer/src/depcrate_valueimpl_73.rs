// Generated macro for impl_73 (impl)
macro_rules! Depcrate_valueimpl_73 {
() => {
// Module: crate::value
// Provides: {"impl_73"}
// Dependencies: {}
impl WriteTomlValue for char { fn write_toml_value < W : TomlWrite + ? Sized > (& self , writer : & mut W) -> core :: fmt :: Result { let mut buf = [0 ; 4] ; let v = self . encode_utf8 (& mut buf) ; v . write_toml_value (writer) } }
};
}
