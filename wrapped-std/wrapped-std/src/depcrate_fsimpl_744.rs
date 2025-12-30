// Generated macro for impl_744 (impl)
macro_rules! Depcrate_fsimpl_744 {
() => {
// Module: crate::fs
// Provides: {"impl_744"}
// Dependencies: {}
# [stable (feature = "io_traits_arc" , since = "1.73.0")] impl Seek for Arc < File > { fn seek (& mut self , pos : SeekFrom) -> io :: Result < u64 > { (& * * self) . seek (pos) } fn stream_len (& mut self) -> io :: Result < u64 > { (& * * self) . stream_len () } fn stream_position (& mut self) -> io :: Result < u64 > { (& * * self) . stream_position () } }
};
}
