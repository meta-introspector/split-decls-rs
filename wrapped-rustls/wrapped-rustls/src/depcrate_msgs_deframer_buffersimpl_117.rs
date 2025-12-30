// Generated macro for impl_117 (impl)
macro_rules! Depcrate_msgs_deframer_buffersimpl_117 {
() => {
// Module: crate::msgs::deframer::buffers
// Provides: {"impl_117"}
// Dependencies: {}
impl DeframerVecBuffer { # [doc = " Discard `taken` bytes from the start of our buffer."] pub (crate) fn discard (& mut self , taken : usize) { if taken < self . used { self . buf . copy_within (taken .. self . used , 0) ; self . used -= taken ; } else if taken >= self . used { self . used = 0 ; } } pub (crate) fn filled_mut (& mut self) -> & mut [u8] { & mut self . buf [.. self . used] } pub (crate) fn filled (& self) -> & [u8] { & self . buf [.. self . used] } }
};
}
