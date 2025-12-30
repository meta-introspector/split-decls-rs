// Generated macro for ScmRights (struct)
macro_rules! Depcrate_os_unix_net_ancillaryScmRights {
() => {
// Module: crate::os::unix::net::ancillary
// Provides: {"ScmRights"}
// Dependencies: {}
# [doc = " This control message contains file descriptors."] # [doc = ""] # [doc = " The level is equal to `SOL_SOCKET` and the type is equal to `SCM_RIGHTS`."] # [unstable (feature = "unix_socket_ancillary_data" , issue = "76915")] pub struct ScmRights < 'a > (AncillaryDataIter < 'a , RawFd >) ;
};
}
