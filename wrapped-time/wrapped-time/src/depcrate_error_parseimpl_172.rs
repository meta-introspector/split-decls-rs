// Generated macro for impl_172 (impl)
macro_rules! Depcrate_error_parseimpl_172 {
() => {
// Module: crate::error::parse
// Provides: {"impl_172"}
// Dependencies: {}
impl core :: error :: Error for Parse { # [inline] fn source (& self) -> Option < & (dyn core :: error :: Error + 'static) > { match self { Self :: TryFromParsed (err) => Some (err) , Self :: ParseFromDescription (err) => Some (err) , # [allow (deprecated)] Self :: UnexpectedTrailingCharacters { never } => match * never { } , } } }
};
}
