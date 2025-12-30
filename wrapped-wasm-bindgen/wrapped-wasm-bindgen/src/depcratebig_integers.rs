// Generated macro for big_integers (macro)
macro_rules! Depcratebig_integers {
() => {
// Module: crate
// Provides: {"big_integers"}
// Dependencies: {}
macro_rules ! big_integers { ($ ($ n : ident) *) => ($ (impl PartialEq <$ n > for JsValue { # [inline] fn eq (& self , other : &$ n) -> bool { self == & JsValue :: from (* other) } } impl From <$ n > for JsValue { # [inline] fn from (arg : $ n) -> JsValue { wbg_cast (arg) } } impl TryFrom < JsValue > for $ n { type Error = JsValue ; # [inline] fn try_from (v : JsValue) -> Result < Self , JsValue > { Self :: try_from_js_value (v) } } impl TryFromJsValue for $ n { # [inline] fn try_from_js_value_ref (val : & JsValue) -> Option <$ n > { let as_i64 = __wbindgen_bigint_get_as_i64 (& val) ?; let as_self = as_i64 as $ n ; if val == & as_self { Some (as_self) } else { None } } }) *) }
};
}
