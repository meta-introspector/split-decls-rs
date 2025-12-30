// Generated macro for impl_741 (impl)
macro_rules! Depcrate_fsimpl_741 {
() => {
// Module: crate::fs
// Provides: {"impl_741"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl Seek for File { fn seek (& mut self , pos : SeekFrom) -> io :: Result < u64 > { (& * self) . seek (pos) } fn stream_len (& mut self) -> io :: Result < u64 > { (& * self) . stream_len () } fn stream_position (& mut self) -> io :: Result < u64 > { (& * self) . stream_position () } }
};
}
