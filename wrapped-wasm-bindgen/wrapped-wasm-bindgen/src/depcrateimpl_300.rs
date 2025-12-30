// Generated macro for impl_300 (impl)
macro_rules! Depcrateimpl_300 {
() => {
// Module: crate
// Provides: {"impl_300"}
// Dependencies: {}
impl < T > From < NonNull < T > > for JsValue { # [inline] fn from (s : NonNull < T >) -> JsValue { JsValue :: from (s . as_ptr () as usize) } }
};
}
