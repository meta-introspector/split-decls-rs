// Generated macro for impl_418 (impl)
macro_rules! Depcrate_stream_recoverableimpl_418 {
() => {
// Module: crate::stream::recoverable
// Provides: {"impl_418"}
// Dependencies: {}
impl < I , E > core :: ops :: Deref for Recoverable < I , E > where I : Stream , { type Target = I ; # [inline (always)] fn deref (& self) -> & Self :: Target { & self . input } }
};
}
