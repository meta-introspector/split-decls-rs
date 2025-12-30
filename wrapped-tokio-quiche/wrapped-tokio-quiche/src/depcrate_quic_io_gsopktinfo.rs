// Generated macro for PktInfo (enum)
macro_rules! Depcrate_quic_io_gsoPktInfo {
() => {
// Module: crate::quic::io::gso
// Provides: {"PktInfo"}
// Dependencies: {}
# [cfg (all (target_os = "linux" , not (feature = "fuzzing")))] # [derive (Copy , Clone , Debug)] pub (crate) enum PktInfo { V4 (libc :: in_pktinfo) , V6 (libc :: in6_pktinfo) , }
};
}
