// Generated macro for impl_895 (impl)
macro_rules! Depcrate_net_send_recv_msgimpl_895 {
() => {
// Module: crate::net::send_recv::msg
// Provides: {"impl_895"}
// Dependencies: {}
impl < 'buf > From < & 'buf mut [MaybeUninit < u8 >] > for RecvAncillaryBuffer < 'buf > { fn from (buffer : & 'buf mut [MaybeUninit < u8 >]) -> Self { Self :: new (buffer) } }
};
}
