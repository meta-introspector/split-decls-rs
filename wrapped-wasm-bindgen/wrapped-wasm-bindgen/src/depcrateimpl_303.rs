// Generated macro for impl_303 (impl)
macro_rules! Depcrateimpl_303 {
() => {
// Module: crate
// Provides: {"impl_303"}
// Dependencies: {}
impl TryFrom < JsValue > for String { type Error = JsValue ; fn try_from (value : JsValue) -> Result < Self , Self :: Error > { match value . as_string () { Some (s) => Ok (s) , None => Err (value) , } } }
};
}
