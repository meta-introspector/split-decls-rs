// Generated macro for impl_134 (impl)
macro_rules! Depcrate_stream_closeimpl_134 {
() => {
// Module: crate::stream_close
// Provides: {"impl_134"}
// Dependencies: {}
impl < S > StreamNotifyClose < S > { # [doc = " Create a new `StreamNotifyClose`."] pub fn new (stream : S) -> Self { Self { inner : Some (stream) , } } # [doc = " Get back the inner `Stream`."] # [doc = ""] # [doc = " Returns `None` if the stream has reached its end."] pub fn into_inner (self) -> Option < S > { self . inner } }
};
}
