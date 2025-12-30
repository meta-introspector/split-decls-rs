// Generated macro for impl_111 (impl)
macro_rules! Depcrate_msgs_deframer_buffersimpl_111 {
() => {
// Module: crate::msgs::deframer::buffers
// Provides: {"impl_111"}
// Dependencies: {}
impl < 'b > Delocator < 'b > { # [inline] pub (crate) fn new (slice : & 'b [u8]) -> Self { Self { slice } } # [inline] pub (crate) fn slice_from_range (& '_ self , range : & Range < usize >) -> & 'b [u8] { self . slice . get (range . clone ()) . unwrap () } # [inline] pub (crate) fn locator (self) -> Locator { Locator :: new (self . slice) } }
};
}
