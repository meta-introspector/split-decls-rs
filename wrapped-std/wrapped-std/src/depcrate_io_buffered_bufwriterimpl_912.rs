// Generated macro for impl_912 (impl)
macro_rules! Depcrate_io_buffered_bufwriterimpl_912 {
() => {
// Module: crate::io::buffered::bufwriter
// Provides: {"impl_912"}
// Dependencies: {}
impl WriterPanicked { # [doc = " Returns the perhaps-unwritten data.  Some of this data may have been written by the"] # [doc = " panicking call(s) to the underlying writer, so simply writing it again is not a good idea."] # [must_use = "`self` will be dropped if the result is not used"] # [stable (feature = "bufwriter_into_parts" , since = "1.56.0")] pub fn into_inner (self) -> Vec < u8 > { self . buf } }
};
}
