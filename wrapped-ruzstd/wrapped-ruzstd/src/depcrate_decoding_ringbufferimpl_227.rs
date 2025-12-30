// Generated macro for impl_227 (impl)
macro_rules! Depcrate_decoding_ringbufferimpl_227 {
() => {
// Module: crate::decoding::ringbuffer
// Provides: {"impl_227"}
// Dependencies: {}
impl Drop for RingBuffer { fn drop (& mut self) { if self . cap == 0 { return ; } let current_layout = unsafe { Layout :: array :: < u8 > (self . cap) . unwrap_unchecked () } ; unsafe { dealloc (self . buf . as_ptr () , current_layout) ; } } }
};
}
