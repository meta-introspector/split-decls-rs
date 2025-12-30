// Generated macro for impl_321 (impl)
macro_rules! Depcrate_ring_bufferimpl_321 {
() => {
// Module: crate::ring_buffer
// Provides: {"impl_321"}
// Dependencies: {}
# [cfg (feature = "std")] impl < const N : usize > std :: io :: Read for RingBuffer < u8 , N > { fn read (& mut self , buf : & mut [u8]) -> std :: io :: Result < usize > { let read_size = buf . len () . min (self . len ()) ; if read_size == 0 { Ok (0) } else { for p in buf . iter_mut () . take (read_size) { * p = self . pop_front () . unwrap () ; } Ok (read_size) } } }
};
}
