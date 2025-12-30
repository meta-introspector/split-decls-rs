// Generated macro for impl_36 (impl)
macro_rules! Depcrateimpl_36 {
() => {
// Module: crate
// Provides: {"impl_36"}
// Dependencies: {}
impl fmt :: Display for ParseSignatureError { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { match self { ParseSignatureError :: WrongSize => { f . write_str ("string decoded to wrong size for signature") } ParseSignatureError :: Invalid => f . write_str ("failed to decode string to signature") , } } }
};
}
