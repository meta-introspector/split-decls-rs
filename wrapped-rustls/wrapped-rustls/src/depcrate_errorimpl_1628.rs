// Generated macro for impl_1628 (impl)
macro_rules! Depcrate_errorimpl_1628 {
() => {
// Module: crate::error
// Provides: {"impl_1628"}
// Dependencies: {}
# [cfg (feature = "std")] impl From < SystemTimeError > for Error { # [inline] fn from (_ : SystemTimeError) -> Self { Self :: FailedToGetCurrentTime } }
};
}
