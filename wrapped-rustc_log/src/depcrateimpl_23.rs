// Generated macro for impl_23 (impl)
macro_rules! Depcrateimpl_23 {
() => {
// Module: crate
// Provides: {"impl_23"}
// Dependencies: {}
impl Display for Error { fn fmt (& self , formatter : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Error :: InvalidColorValue (value) => write ! (formatter , "invalid log color value '{value}': expected one of always, never, or auto" ,) , Error :: NonUnicodeColorValue => write ! (formatter , "non-Unicode log color value: expected one of always, never, or auto" ,) , Error :: InvalidWraptree (value) => write ! (formatter , "invalid log WRAPTREE value '{value}': expected a non-negative integer" ,) , Error :: AlreadyInit (tracing_error) => Display :: fmt (tracing_error , formatter) , } } }
};
}
