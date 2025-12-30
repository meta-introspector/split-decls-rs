// Generated macro for impl_271 (impl)
macro_rules! Depcrateimpl_271 {
() => {
// Module: crate
// Provides: {"impl_271"}
// Dependencies: {}
impl Not for & JsValue { type Output = bool ; # [doc = " Applies the `!` JS operator on a `JsValue`."] # [doc = ""] # [doc = " [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Logical_NOT)"] # [inline] fn not (self) -> Self :: Output { JsValue :: is_falsy (self) } }
};
}
