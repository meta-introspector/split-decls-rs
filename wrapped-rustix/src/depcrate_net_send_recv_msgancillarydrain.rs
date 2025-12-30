// Generated macro for AncillaryDrain (struct)
macro_rules! Depcrate_net_send_recv_msgAncillaryDrain {
() => {
// Module: crate::net::send_recv::msg
// Provides: {"AncillaryDrain"}
// Dependencies: {}
# [doc = " An iterator that drains messages from a [`RecvAncillaryBuffer`]."] pub struct AncillaryDrain < 'buf > { # [doc = " Inner iterator over messages."] messages : messages :: Messages < 'buf > , # [doc = " Increment the number of messages we've read."] # [doc = " Decrement the total length."] read_and_length : Option < (& 'buf mut usize , & 'buf mut usize) > , }
};
}
