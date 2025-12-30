// Generated macro for impl_763 (impl)
macro_rules! Depcrate_replyimpl_763 {
() => {
// Module: crate::reply
// Provides: {"impl_763"}
// Dependencies: {}
impl < T : Send > Reply for :: http :: Response < T > where crate :: bodyt :: Body : From < T > , { # [inline] fn into_response (self) -> Response { self . map (Body :: from) } }
};
}
