// Generated macro for impl_11 (impl)
macro_rules! Depcrate_errorsimpl_11 {
() => {
// Module: crate::errors
// Provides: {"impl_11"}
// Dependencies: {}
impl TryFrom < JsValue > for JsError { type Error = NotJsError ; fn try_from (value : JsValue) -> Result < Self , Self :: Error > { match value . dyn_into :: < js_sys :: Error > () { Ok (error) => Ok (JsError :: from (error)) , Err (js_value) => { let js_to_string = String :: from (js_sys :: JsString :: from (js_value . clone ())) ; Err (NotJsError { js_value , js_to_string , }) } } } }
};
}
