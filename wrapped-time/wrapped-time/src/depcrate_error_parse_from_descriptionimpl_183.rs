// Generated macro for impl_183 (impl)
macro_rules! Depcrate_error_parse_from_descriptionimpl_183 {
() => {
// Module: crate::error::parse_from_description
// Provides: {"impl_183"}
// Dependencies: {}
impl fmt :: Display for ParseFromDescription { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Self :: InvalidLiteral => f . write_str ("a character literal was not valid") , Self :: InvalidComponent (name) => { write ! (f , "the '{name}' component could not be parsed") } Self :: UnexpectedTrailingCharacters => { f . write_str ("unexpected trailing characters; the end of input was expected") } } } }
};
}
