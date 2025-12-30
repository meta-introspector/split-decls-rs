// Generated macro for SendAncillaryBuffer (struct)
macro_rules! Depcrate_net_send_recv_msgSendAncillaryBuffer {
() => {
// Module: crate::net::send_recv::msg
// Provides: {"SendAncillaryBuffer"}
// Dependencies: {}
# [doc = " Buffer for sending ancillary messages with [`sendmsg`] and"] # [doc = " [`sendmsg_addr`]."] # [doc = ""] # [doc = " Use the [`push`] function to add messages to send."] # [doc = ""] # [doc = " [`push`]: SendAncillaryBuffer::push"] pub struct SendAncillaryBuffer < 'buf , 'slice , 'fd > { # [doc = " Raw byte buffer for messages."] buffer : & 'buf mut [MaybeUninit < u8 >] , # [doc = " The amount of the buffer that is used."] length : usize , # [doc = " Phantom data for lifetime of `&'slice [BorrowedFd<'fd>]`."] _phantom : PhantomData < & 'slice [BorrowedFd < 'fd >] > , }
};
}
