// Generated macro for impl_145 (impl)
macro_rules! Depcrate_msgs_deframerimpl_145 {
() => {
// Module: crate::msgs::deframer
// Provides: {"impl_145"}
// Dependencies: {}
impl < 'a > DeframerIter < 'a > { # [doc = " Make a new `DeframerIter`"] pub (crate) fn new (buf : & 'a mut [u8]) -> Self { Self { buf , consumed : 0 } } # [doc = " How many bytes were processed successfully from the front"] # [doc = " of the buffer passed to `new()`?"] pub (crate) fn bytes_consumed (& self) -> usize { self . consumed } }
};
}
