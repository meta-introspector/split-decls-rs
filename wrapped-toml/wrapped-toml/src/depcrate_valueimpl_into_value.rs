// Generated macro for impl_into_value (macro)
macro_rules! Depcrate_valueimpl_into_value {
() => {
// Module: crate::value
// Provides: {"impl_into_value"}
// Dependencies: {}
macro_rules ! impl_into_value { ($ variant : ident : $ T : ty) => { impl From <$ T > for Value { # [inline] fn from (val : $ T) -> Value { Value ::$ variant (val . into ()) } } } ; }
};
}
