// Generated macro for impl_72 (impl)
macro_rules! Depcrate_valueimpl_72 {
() => {
// Module: crate::value
// Provides: {"impl_72"}
// Dependencies: {}
impl WriteTomlValue for f64 { fn write_toml_value < W : TomlWrite + ? Sized > (& self , writer : & mut W) -> core :: fmt :: Result { match (self . is_sign_negative () , self . is_nan () , * self == 0.0) { (true , true , _) => write ! (writer , "-nan") , (false , true , _) => write ! (writer , "nan") , (true , false , true) => write ! (writer , "-0.0") , (false , false , true) => write ! (writer , "0.0") , (_ , false , false) => { if self % 1.0 == 0.0 { write ! (writer , "{self}.0") } else { write ! (writer , "{self}") } } } } }
};
}
