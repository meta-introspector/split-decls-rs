// Generated macro for impl_177 (impl)
macro_rules! Depcrate_error_parseimpl_177 {
() => {
// Module: crate::error::parse
// Provides: {"impl_177"}
// Dependencies: {}
impl From < Parse > for crate :: Error { # [inline] fn from (err : Parse) -> Self { match err { Parse :: TryFromParsed (err) => Self :: TryFromParsed (err) , Parse :: ParseFromDescription (err) => Self :: ParseFromDescription (err) , # [allow (deprecated)] Parse :: UnexpectedTrailingCharacters { never } => match never { } , } } }
};
}
