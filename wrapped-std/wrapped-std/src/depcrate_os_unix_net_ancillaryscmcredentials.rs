// Generated macro for ScmCredentials (struct)
macro_rules! Depcrate_os_unix_net_ancillaryScmCredentials {
() => {
// Module: crate::os::unix::net::ancillary
// Provides: {"ScmCredentials"}
// Dependencies: {}
# [cfg (target_os = "netbsd")] # [unstable (feature = "unix_socket_ancillary_data" , issue = "76915")] pub struct ScmCredentials < 'a > (AncillaryDataIter < 'a , libc :: sockcred >) ;
};
}
