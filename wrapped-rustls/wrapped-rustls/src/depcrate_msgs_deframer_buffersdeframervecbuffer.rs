// Generated macro for DeframerVecBuffer (struct)
macro_rules! Depcrate_msgs_deframer_buffersDeframerVecBuffer {
() => {
// Module: crate::msgs::deframer::buffers
// Provides: {"DeframerVecBuffer"}
// Dependencies: {}
# [derive (Default , Debug)] pub (crate) struct DeframerVecBuffer { # [doc = " Buffer of data read from the socket, in the process of being parsed into messages."] # [doc = ""] # [doc = " For buffer size management, checkout out the [`DeframerVecBuffer::prepare_read()`] method."] buf : Vec < u8 > , # [doc = " What size prefix of `buf` is used."] used : usize , }
};
}
