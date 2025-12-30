// Generated macro for DeframerSliceBuffer (struct)
macro_rules! Depcrate_msgs_deframer_buffersDeframerSliceBuffer {
() => {
// Module: crate::msgs::deframer::buffers
// Provides: {"DeframerSliceBuffer"}
// Dependencies: {}
# [doc = " A borrowed version of [`DeframerVecBuffer`] that tracks discard operations"] # [derive (Debug)] pub (crate) struct DeframerSliceBuffer < 'a > { buf : & 'a mut [u8] , discard : usize , }
};
}
