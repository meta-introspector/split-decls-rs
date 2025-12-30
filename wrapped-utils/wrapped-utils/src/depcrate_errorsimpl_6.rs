// Generated macro for impl_6 (impl)
macro_rules! Depcrate_errorsimpl_6 {
() => {
// Module: crate::errors
// Provides: {"impl_6"}
// Dependencies: {}
impl From < js_sys :: Error > for JsError { fn from (error : js_sys :: Error) -> Self { JsError { name : String :: from (error . name ()) , message : String :: from (error . message ()) , js_to_string : String :: from (error . to_string ()) , } } }
};
}
