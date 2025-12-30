// Generated macro for num128 (macro)
macro_rules! Depcratenum128 {
() => {
// Module: crate
// Provides: {"num128"}
// Dependencies: {}
macro_rules ! num128 { ($ ty : ty , $ hi_ty : ty) => { impl PartialEq <$ ty > for JsValue { # [inline] fn eq (& self , other : &$ ty) -> bool { self == & JsValue :: from (* other) } } impl From <$ ty > for JsValue { # [inline] fn from (arg : $ ty) -> JsValue { wbg_cast (arg) } } impl TryFrom < JsValue > for $ ty { type Error = JsValue ; # [inline] fn try_from (v : JsValue) -> Result < Self , JsValue > { Self :: try_from_js_value (v) } } impl TryFromJsValue for $ ty { fn try_from_js_value_ref (v : & JsValue) -> Option <$ ty > { let lo = __wbindgen_bigint_get_as_i64 (& v) ? as u64 ; let hi = v >> JsValue :: from (64_u64) ; <$ hi_ty >:: try_from_js_value_ref (& hi) . map (| hi | Self :: from (hi) << 64 | Self :: from (lo)) } } } ; }
};
}
