// Generated macro for impl_120 (impl)
macro_rules! Depcrate_msgs_deframer_buffersimpl_120 {
() => {
// Module: crate::msgs::deframer::buffers
// Provides: {"impl_120"}
// Dependencies: {}
impl < 'a > DeframerSliceBuffer < 'a > { pub (crate) fn new (buf : & 'a mut [u8]) -> Self { Self { buf , discard : 0 } } # [doc = " Tracks a pending discard operation of `num_bytes`"] pub (crate) fn queue_discard (& mut self , num_bytes : usize) { self . discard += num_bytes ; } pub (crate) fn pending_discard (& self) -> usize { self . discard } pub (crate) fn filled_mut (& mut self) -> & mut [u8] { & mut self . buf [self . discard ..] } }
};
}
