// Generated macro for impl_887 (impl)
macro_rules! Depcrate_net_send_recv_msgimpl_887 {
() => {
// Module: crate::net::send_recv::msg
// Provides: {"impl_887"}
// Dependencies: {}
impl SendAncillaryMessage < '_ , '_ > { # [doc = " Get the maximum size of an ancillary message."] # [doc = ""] # [doc = " This can be used to determine the size of the buffer to allocate for a"] # [doc = " [`SendAncillaryBuffer::new`] with one message."] pub const fn size (& self) -> usize { match self { Self :: ScmRights (slice) => cmsg_space ! (ScmRights (slice . len ())) , # [cfg (linux_kernel)] Self :: ScmCredentials (_) => cmsg_space ! (ScmCredentials (1)) , # [cfg (target_os = "linux")] Self :: TxTime (_) => cmsg_space ! (TxTime (1)) , } } }
};
}
