// Generated macro for impl_46 (impl)
macro_rules! Depcrate_bufreadimpl_46 {
() => {
// Module: crate::bufread
// Provides: {"impl_46"}
// Dependencies: {}
impl < R : BufRead > XzEncoder < R > { # [doc = " Creates a new encoder which will read uncompressed data from the given"] # [doc = " stream and emit the compressed stream."] # [doc = ""] # [doc = " The `level` argument here is typically 0-9 with 6 being a good default."] pub fn new (r : R , level : u32) -> XzEncoder < R > { let stream = Stream :: new_easy_encoder (level , Check :: Crc64) . unwrap () ; XzEncoder :: new_stream (r , stream) } # [doc = " Creates a new encoder with a custom `Stream`."] # [doc = ""] # [doc = " The `Stream` can be pre-configured for multithreaded encoding, different"] # [doc = " compression options/tuning, etc."] pub fn new_stream (r : R , stream : Stream) -> XzEncoder < R > { XzEncoder { obj : r , data : stream , } } }
};
}
