// Generated macro for impl_2169 (impl)
macro_rules! Depcrate_time_providerimpl_2169 {
() => {
// Module: crate::time_provider
// Provides: {"impl_2169"}
// Dependencies: {}
# [cfg (feature = "std")] impl TimeProvider for DefaultTimeProvider { fn current_time (& self) -> Option < UnixTime > { Some (UnixTime :: now ()) } }
};
}
