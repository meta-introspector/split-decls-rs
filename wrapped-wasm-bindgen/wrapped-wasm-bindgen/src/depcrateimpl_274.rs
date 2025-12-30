// Generated macro for impl_274 (impl)
macro_rules! Depcrateimpl_274 {
() => {
// Module: crate
// Provides: {"impl_274"}
// Dependencies: {}
impl TryFrom < & JsValue > for f64 { type Error = JsValue ; # [doc = " Applies the unary `+` JS operator on a `JsValue`."] # [doc = " Returns the numeric result on success, or the JS error value on error."] # [doc = ""] # [doc = " [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Unary_plus)"] # [inline] fn try_from (val : & JsValue) -> Result < Self , Self :: Error > { let jsval = __wbindgen_try_into_number (val) ; match jsval . as_f64 () { Some (num) => Ok (num) , None => Err (jsval) , } } }
};
}
