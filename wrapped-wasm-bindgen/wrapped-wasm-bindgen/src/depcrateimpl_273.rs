// Generated macro for impl_273 (impl)
macro_rules! Depcrateimpl_273 {
() => {
// Module: crate
// Provides: {"impl_273"}
// Dependencies: {}
impl TryFrom < JsValue > for f64 { type Error = JsValue ; # [doc = " Applies the unary `+` JS operator on a `JsValue`."] # [doc = " Returns the numeric result on success, or the JS error value on error."] # [doc = ""] # [doc = " [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Unary_plus)"] # [inline] fn try_from (val : JsValue) -> Result < Self , Self :: Error > { f64 :: try_from (& val) } }
};
}
