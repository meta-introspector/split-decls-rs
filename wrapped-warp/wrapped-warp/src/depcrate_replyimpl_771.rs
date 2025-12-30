// Generated macro for impl_771 (impl)
macro_rules! Depcrate_replyimpl_771 {
() => {
// Module: crate::reply
// Provides: {"impl_771"}
// Dependencies: {}
impl Reply for Cow < 'static , str > { # [inline] fn into_response (self) -> Response { match self { Cow :: Borrowed (s) => s . into_response () , Cow :: Owned (s) => s . into_response () , } } }
};
}
