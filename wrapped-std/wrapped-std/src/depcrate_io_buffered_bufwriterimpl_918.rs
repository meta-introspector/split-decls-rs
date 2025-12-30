// Generated macro for impl_918 (impl)
macro_rules! Depcrate_io_buffered_bufwriterimpl_918 {
() => {
// Module: crate::io::buffered::bufwriter
// Provides: {"impl_918"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < W : ? Sized + Write + Seek > Seek for BufWriter < W > { # [doc = " Seek to the offset, in bytes, in the underlying writer."] # [doc = ""] # [doc = " Seeking always writes out the internal buffer before seeking."] fn seek (& mut self , pos : SeekFrom) -> io :: Result < u64 > { self . flush_buf () ? ; self . get_mut () . seek (pos) } }
};
}
