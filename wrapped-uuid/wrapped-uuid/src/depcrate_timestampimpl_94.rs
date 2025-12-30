// Generated macro for impl_94 (impl)
macro_rules! Depcrate_timestampimpl_94 {
() => {
// Module: crate::timestamp
// Provides: {"impl_94"}
// Dependencies: {}
# [cfg (feature = "std")] impl From < Timestamp > for std :: time :: SystemTime { fn from (ts : Timestamp) -> Self { let (seconds , subsec_nanos) = ts . to_unix () ; Self :: UNIX_EPOCH + std :: time :: Duration :: new (seconds , subsec_nanos) } }
};
}
