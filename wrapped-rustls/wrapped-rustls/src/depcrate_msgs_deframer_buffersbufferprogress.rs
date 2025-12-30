// Generated macro for BufferProgress (struct)
macro_rules! Depcrate_msgs_deframer_buffersBufferProgress {
() => {
// Module: crate::msgs::deframer::buffers
// Provides: {"BufferProgress"}
// Dependencies: {}
# [doc = " Accounting structure tracking progress in parsing a single buffer."] # [derive (Clone , Debug)] pub (crate) struct BufferProgress { # [doc = " Prefix of the buffer that has been processed so far."] # [doc = ""] # [doc = " `processed` may exceed `discard`, that means we have parsed"] # [doc = " some buffer, but are still using it.  This happens due to"] # [doc = " in-place decryption of incoming records, and in-place"] # [doc = " reassembly of handshake messages."] # [doc = ""] # [doc = " 0 <= processed <= len"] processed : usize , # [doc = " Prefix of the buffer that can be removed."] # [doc = ""] # [doc = " If `discard` exceeds `processed`, that means we are ignoring"] # [doc = " data without processing it."] # [doc = ""] # [doc = " 0 <= discard <= len"] discard : usize , }
};
}
