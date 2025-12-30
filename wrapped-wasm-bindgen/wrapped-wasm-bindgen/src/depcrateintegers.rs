// Generated macro for integers (macro)
macro_rules! Depcrateintegers {
() => {
// Module: crate
// Provides: {"integers"}
// Dependencies: {}
macro_rules ! integers { ($ ($ n : ident) *) => ($ (impl PartialEq <$ n > for JsValue { # [inline] fn eq (& self , other : &$ n) -> bool { self . as_f64 () == Some (f64 :: from (* other)) } } impl From <$ n > for JsValue { # [inline] fn from (n : $ n) -> JsValue { JsValue :: from_f64 (n . into ()) } } impl TryFromJsValue for $ n { # [inline] fn try_from_js_value_ref (val : & JsValue) -> Option <$ n > { to_uint_32 (val) . map (| n | n as $ n) } }) *) }
};
}
