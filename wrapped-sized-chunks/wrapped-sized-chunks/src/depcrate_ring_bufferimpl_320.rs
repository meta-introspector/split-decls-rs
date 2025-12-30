// Generated macro for impl_320 (impl)
macro_rules! Depcrate_ring_bufferimpl_320 {
() => {
// Module: crate::ring_buffer
// Provides: {"impl_320"}
// Dependencies: {}
# [cfg (feature = "std")] impl < const N : usize > std :: io :: Write for RingBuffer < u8 , N > { fn write (& mut self , mut buf : & [u8]) -> std :: io :: Result < usize > { let max_new = Self :: CAPACITY - self . len () ; if buf . len () > max_new { buf = & buf [.. max_new] ; } unsafe { self . copy_from_slice (buf , self . origin + self . len ()) } ; self . length += buf . len () ; Ok (buf . len ()) } # [inline] fn flush (& mut self) -> std :: io :: Result < () > { Ok (()) } }
};
}
