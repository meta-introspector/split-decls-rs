// Generated macro for impl_115 (impl)
macro_rules! Depcrate_msgs_deframer_buffersimpl_115 {
() => {
// Module: crate::msgs::deframer::buffers
// Provides: {"impl_115"}
// Dependencies: {}
impl BufferProgress { pub (super) fn new (processed : usize) -> Self { Self { processed , discard : 0 , } } # [inline] pub (crate) fn add_discard (& mut self , discard : usize) { self . discard += discard ; } # [inline] pub (crate) fn add_processed (& mut self , processed : usize) { self . processed += processed ; } # [inline] pub (crate) fn take_discard (& mut self) -> usize { self . processed = self . processed . saturating_sub (self . discard) ; mem :: take (& mut self . discard) } # [inline] pub (crate) fn processed (& self) -> usize { self . processed } }
};
}
