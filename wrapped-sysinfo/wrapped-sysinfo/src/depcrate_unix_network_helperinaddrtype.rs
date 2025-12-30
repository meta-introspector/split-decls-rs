// Generated macro for InAddrType (type)
macro_rules! Depcrate_unix_network_helperInAddrType {
() => {
// Module: crate::unix::network_helper
// Provides: {"InAddrType"}
// Dependencies: {}
# [cfg (any (target_os = "illumos" , target_os = "solaris"))] pub type InAddrType = libc :: c_ulonglong ;
};
}
