// Generated macro for impl_4 (impl)
macro_rules! Depcrateimpl_4 {
() => {
// Module: crate
// Provides: {"impl_4"}
// Dependencies: {}
impl fmt :: Display for SanitizeError { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { match self { SanitizeError :: IndexOutOfBounds => f . write_str ("index out of bounds") , SanitizeError :: ValueOutOfBounds => f . write_str ("value out of bounds") , SanitizeError :: InvalidValue => f . write_str ("invalid value") , } } }
};
}
