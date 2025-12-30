// Generated macro for js_to_error (function)
macro_rules! Depcrate_errorsjs_to_error {
() => {
// Module: crate::errors
// Provides: {"js_to_error"}
// Dependencies: {}
pub (crate) fn js_to_error (js_value : JsValue) -> StorageError { match js_value . dyn_into :: < js_sys :: Error > () { Ok (error) => StorageError :: JsError (JsError :: from (error)) , Err (_) => unreachable ! ("JsValue passed is not an Error type - this is a bug") , } }
};
}
