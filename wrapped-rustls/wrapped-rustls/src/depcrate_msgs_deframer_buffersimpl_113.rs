// Generated macro for impl_113 (impl)
macro_rules! Depcrate_msgs_deframer_buffersimpl_113 {
() => {
// Module: crate::msgs::deframer::buffers
// Provides: {"impl_113"}
// Dependencies: {}
impl < 'b > Coalescer < 'b > { # [inline] pub (crate) fn new (slice : & 'b mut [u8]) -> Self { Self { slice } } # [inline] pub (crate) fn copy_within (& mut self , from : Range < usize > , to : Range < usize >) { debug_assert ! (from . len () == to . len ()) ; debug_assert ! (self . slice . get (from . clone ()) . is_some ()) ; debug_assert ! (self . slice . get (to . clone ()) . is_some ()) ; self . slice . copy_within (from , to . start) ; } # [inline] pub (crate) fn delocator (self) -> Delocator < 'b > { Delocator :: new (self . slice) } }
};
}
