// Generated macro for impl_16 (impl)
macro_rules! Depcrate_errorimpl_16 {
() => {
// Module: crate::error
// Provides: {"impl_16"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl From < Box < dyn core :: error :: Error + Send + Sync + 'static > > for Error { fn from (source : Box < dyn core :: error :: Error + Send + Sync + 'static >) -> Error { Self :: from_source (source) } }
};
}
