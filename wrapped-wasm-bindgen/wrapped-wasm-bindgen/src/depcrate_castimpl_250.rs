// Generated macro for impl_250 (impl)
macro_rules! Depcrate_castimpl_250 {
() => {
// Module: crate::cast
// Provides: {"impl_250"}
// Dependencies: {}
impl < T : JsCast > TryFromJsValue for T { # [inline] fn try_from_js_value (val : JsValue) -> Result < Self , JsValue > { val . dyn_into () } # [inline] fn try_from_js_value_ref (val : & JsValue) -> Option < Self > { val . clone () . dyn_into () . ok () } }
};
}
