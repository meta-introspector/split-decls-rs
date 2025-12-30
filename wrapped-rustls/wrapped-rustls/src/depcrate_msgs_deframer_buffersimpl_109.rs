// Generated macro for impl_109 (impl)
macro_rules! Depcrate_msgs_deframer_buffersimpl_109 {
() => {
// Module: crate::msgs::deframer::buffers
// Provides: {"impl_109"}
// Dependencies: {}
impl Locator { # [inline] pub (crate) fn new (slice : & [u8]) -> Self { Self { bounds : slice . as_ptr_range () , } } # [inline] pub (crate) fn locate (& self , slice : & [u8]) -> Range < usize > { let bounds = slice . as_ptr_range () ; debug_assert ! (self . fully_contains (slice)) ; let start = bounds . start as usize - self . bounds . start as usize ; let len = bounds . end as usize - bounds . start as usize ; Range { start , end : start + len , } } # [inline] pub (crate) fn fully_contains (& self , slice : & [u8]) -> bool { let bounds = slice . as_ptr_range () ; bounds . start >= self . bounds . start && bounds . end <= self . bounds . end } }
};
}
