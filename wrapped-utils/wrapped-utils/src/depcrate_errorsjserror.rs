// Generated macro for JsError (struct)
macro_rules! Depcrate_errorsJsError {
() => {
// Module: crate::errors
// Provides: {"JsError"}
// Dependencies: {}
# [doc = " Wrapper type around [`js_sys::Error`]"] # [doc = ""] # [doc = " [`Display`][fmt::Display] impl returns the result `error.toString()` from JavaScript"] pub struct JsError { # [doc = " `name` from [`js_sys::Error`]"] pub name : String , # [doc = " `message` from [`js_sys::Error`]"] pub message : String , js_to_string : String , }
};
}
