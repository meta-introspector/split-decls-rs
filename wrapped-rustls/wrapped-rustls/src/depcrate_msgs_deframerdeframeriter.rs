// Generated macro for DeframerIter (struct)
macro_rules! Depcrate_msgs_deframerDeframerIter {
() => {
// Module: crate::msgs::deframer
// Provides: {"DeframerIter"}
// Dependencies: {}
# [doc = " A deframer of TLS wire messages."] # [doc = ""] # [doc = " Returns `Some(Ok(_))` containing each `InboundOpaqueMessage` deframed"] # [doc = " from the buffer."] # [doc = ""] # [doc = " Returns `None` if no further messages can be deframed from the"] # [doc = " buffer.  More data is required for further progress."] # [doc = ""] # [doc = " Returns `Some(Err(_))` if the peer is not talking TLS, but some"] # [doc = " other protocol.  The caller should abort the connection, because"] # [doc = " the deframer cannot recover."] # [doc = ""] # [doc = " Call `bytes_consumed()` to learn how many bytes the iterator has"] # [doc = " processed from the front of the original buffer.  This is only updated"] # [doc = " when a message is successfully deframed (ie. `Some(Ok(_))` is returned)."] pub (crate) struct DeframerIter < 'a > { buf : & 'a mut [u8] , consumed : usize , }
};
}
