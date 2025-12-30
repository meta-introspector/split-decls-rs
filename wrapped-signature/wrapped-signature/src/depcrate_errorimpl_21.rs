// Generated macro for impl_21 (impl)
macro_rules! Depcrate_errorimpl_21 {
() => {
// Module: crate::error
// Provides: {"impl_21"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl From < Box < dyn core :: error :: Error + Send + Sync + 'static > > for Error { fn from (source : Box < dyn core :: error :: Error + Send + Sync + 'static >) -> Error { Self :: from_source (source) } }
};
}
