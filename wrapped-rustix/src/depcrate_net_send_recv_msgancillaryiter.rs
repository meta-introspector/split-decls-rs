// Generated macro for AncillaryIter (struct)
macro_rules! Depcrate_net_send_recv_msgAncillaryIter {
() => {
// Module: crate::net::send_recv::msg
// Provides: {"AncillaryIter"}
// Dependencies: {}
# [doc = " An iterator over data in an ancillary buffer."] pub struct AncillaryIter < 'data , T > { # [doc = " The data we're iterating over."] data : & 'data mut [u8] , # [doc = " The raw data we're removing."] _marker : PhantomData < T > , }
};
}
