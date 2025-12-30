// Generated macro for impl_75 (impl)
macro_rules! Depcrate_timeimpl_75 {
() => {
// Module: crate::time
// Provides: {"impl_75"}
// Dependencies: {}
# [cfg (feature = "std")] impl TryFrom < std :: time :: SystemTime > for OcspGeneralizedTime { type Error = der :: Error ; fn try_from (other : std :: time :: SystemTime) -> Result < Self , Self :: Error > { Ok (Self (GeneralizedTime :: from_system_time (other) ?)) } }
};
}
