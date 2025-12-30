// Generated macro for RecvAncillaryMessage (enum)
macro_rules! Depcrate_net_send_recv_msgRecvAncillaryMessage {
() => {
// Module: crate::net::send_recv::msg
// Provides: {"RecvAncillaryMessage"}
// Dependencies: {}
# [doc = " Ancillary message for [`recvmsg`]."] # [non_exhaustive] pub enum RecvAncillaryMessage < 'a > { # [doc = " Received file descriptors."] # [doc (alias = "SCM_RIGHTS")] ScmRights (AncillaryIter < 'a , OwnedFd >) , # [doc = " Received process credentials."] # [cfg (linux_kernel)] # [doc (alias = "SCM_CREDENTIALS")] ScmCredentials (UCred) , }
};
}
