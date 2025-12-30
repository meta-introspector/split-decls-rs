// Generated macro for impl_31 (impl)
macro_rules! Depcrateimpl_31 {
() => {
// Module: crate
// Provides: {"impl_31"}
// Dependencies: {}
impl fmt :: Display for Error { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let description = match self { Error :: LengthInvalid => "length invalid" , Error :: NanosInvalid => "invalid number of nanoseconds" , } ; write ! (f , "{description}") } }
};
}
