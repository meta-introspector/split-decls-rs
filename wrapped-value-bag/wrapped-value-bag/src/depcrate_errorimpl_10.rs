// Generated macro for impl_10 (impl)
macro_rules! Depcrate_errorimpl_10 {
() => {
// Module: crate::error
// Provides: {"impl_10"}
// Dependencies: {}
impl Error { # [doc = " Create an error from a message."] pub fn msg (msg : & 'static str) -> Self { Error { inner : Inner :: Msg (msg) , } } # [cfg (feature = "serde1")] pub (crate) fn try_boxed (msg : & 'static str , e : impl fmt :: Display) -> Self { # [cfg (feature = "std")] { Error :: boxed (format ! ("{msg}: {e}")) } # [cfg (not (feature = "std"))] { let _ = e ; Error :: msg (msg) } } }
};
}
