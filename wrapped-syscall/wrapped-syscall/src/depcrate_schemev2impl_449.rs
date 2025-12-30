// Generated macro for impl_449 (impl)
macro_rules! Depcrate_schemev2impl_449 {
() => {
// Module: crate::schemev2
// Provides: {"impl_449"}
// Dependencies: {}
impl CqeOpcode { pub fn try_from_raw (raw : u8) -> Option < Self > { Some (match raw { 0 => Self :: RespondRegular , 1 => Self :: RespondWithFd , 2 => Self :: SendFevent , 3 => Self :: ObtainFd , 4 => Self :: RespondWithMultipleFds , _ => return None , }) } }
};
}
