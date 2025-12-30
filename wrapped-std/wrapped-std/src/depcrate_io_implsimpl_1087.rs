// Generated macro for impl_1087 (impl)
macro_rules! Depcrate_io_implsimpl_1087 {
() => {
// Module: crate::io::impls
// Provides: {"impl_1087"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < S : Seek + ? Sized > Seek for Box < S > { # [inline] fn seek (& mut self , pos : SeekFrom) -> io :: Result < u64 > { (* * self) . seek (pos) } # [inline] fn rewind (& mut self) -> io :: Result < () > { (* * self) . rewind () } # [inline] fn stream_len (& mut self) -> io :: Result < u64 > { (* * self) . stream_len () } # [inline] fn stream_position (& mut self) -> io :: Result < u64 > { (* * self) . stream_position () } # [inline] fn seek_relative (& mut self , offset : i64) -> io :: Result < () > { (* * self) . seek_relative (offset) } }
};
}
