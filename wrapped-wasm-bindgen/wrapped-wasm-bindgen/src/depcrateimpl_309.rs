// Generated macro for impl_309 (impl)
macro_rules! Depcrateimpl_309 {
() => {
// Module: crate
// Provides: {"impl_309"}
// Dependencies: {}
impl < T > From < Option < T > > for JsValue where JsValue : From < T > , { # [inline] fn from (s : Option < T >) -> JsValue { match s { Some (s) => s . into () , None => JsValue :: undefined () , } } }
};
}
