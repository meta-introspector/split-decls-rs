// Generated macro for impl_93 (impl)
macro_rules! Depcrate_runnerimpl_93 {
() => {
// Module: crate::runner
// Provides: {"impl_93"}
// Dependencies: {}
impl FileStatus { fn is_ok (& self) -> bool { match self { Self :: Ok { .. } => true , Self :: Failure (_) | Self :: TypeMismatch { .. } | Self :: LinkMismatch { .. } | Self :: ContentMismatch { .. } => false , } } }
};
}
