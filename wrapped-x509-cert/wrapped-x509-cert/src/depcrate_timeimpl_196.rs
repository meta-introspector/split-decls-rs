// Generated macro for impl_196 (impl)
macro_rules! Depcrate_timeimpl_196 {
() => {
// Module: crate::time
// Provides: {"impl_196"}
// Dependencies: {}
# [cfg (feature = "std")] impl TryFrom < SystemTime > for Time { type Error = der :: Error ; fn try_from (time : SystemTime) -> der :: Result < Time > { let datetime = DateTime :: from_system_time (time) ? ; Ok (datetime . into ()) } }
};
}
