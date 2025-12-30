// Generated macro for impl_1200 (impl)
macro_rules! Depcrate_io_utilimpl_1200 {
() => {
// Module: crate::io::util
// Provides: {"impl_1200"}
// Dependencies: {}
# [stable (feature = "empty_seek" , since = "1.51.0")] impl Seek for Empty { # [inline] fn seek (& mut self , _pos : SeekFrom) -> io :: Result < u64 > { Ok (0) } # [inline] fn stream_len (& mut self) -> io :: Result < u64 > { Ok (0) } # [inline] fn stream_position (& mut self) -> io :: Result < u64 > { Ok (0) } }
};
}
