// Generated macro for impl_308 (impl)
macro_rules! Depcrateimpl_308 {
() => {
// Module: crate
// Provides: {"impl_308"}
// Dependencies: {}
impl < 'a , T > From < & 'a T > for JsValue where T : JsCast , { # [inline] fn from (s : & 'a T) -> JsValue { s . as_ref () . clone () } }
};
}
