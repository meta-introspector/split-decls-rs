// Generated macro for impl_43 (impl)
macro_rules! Depcrate_active_queryimpl_43 {
() => {
// Module: crate::active_query
// Provides: {"impl_43"}
// Dependencies: {}
impl ops :: Deref for QueryStack { type Target = [ActiveQuery] ; # [inline (always)] fn deref (& self) -> & Self :: Target { & self . stack [.. self . len] } }
};
}
