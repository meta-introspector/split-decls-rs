// Generated macro for impl_171 (impl)
macro_rules! Depcrate_error_parseimpl_171 {
() => {
// Module: crate::error::parse
// Provides: {"impl_171"}
// Dependencies: {}
impl fmt :: Display for Parse { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Self :: TryFromParsed (err) => err . fmt (f) , Self :: ParseFromDescription (err) => err . fmt (f) , # [allow (deprecated)] Self :: UnexpectedTrailingCharacters { never } => match * never { } , } } }
};
}
