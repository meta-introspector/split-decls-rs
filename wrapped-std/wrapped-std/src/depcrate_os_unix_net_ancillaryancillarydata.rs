// Generated macro for AncillaryData (enum)
macro_rules! Depcrate_os_unix_net_ancillaryAncillaryData {
() => {
// Module: crate::os::unix::net::ancillary
// Provides: {"AncillaryData"}
// Dependencies: {}
# [doc = " This enum represent one control message of variable type."] # [unstable (feature = "unix_socket_ancillary_data" , issue = "76915")] pub enum AncillaryData < 'a > { ScmRights (ScmRights < 'a >) , # [cfg (any (doc , target_os = "android" , target_os = "linux" , target_os = "netbsd" , target_os = "freebsd" , target_os = "cygwin" ,))] ScmCredentials (ScmCredentials < 'a >) , }
};
}
