// Generated macro for impl_890 (impl)
macro_rules! Depcrate_net_send_recv_msgimpl_890 {
() => {
// Module: crate::net::send_recv::msg
// Provides: {"impl_890"}
// Dependencies: {}
impl < 'buf > From < & 'buf mut [MaybeUninit < u8 >] > for SendAncillaryBuffer < 'buf , '_ , '_ > { fn from (buffer : & 'buf mut [MaybeUninit < u8 >]) -> Self { Self :: new (buffer) } }
};
}
