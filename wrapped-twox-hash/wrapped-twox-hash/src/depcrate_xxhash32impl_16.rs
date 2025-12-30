// Generated macro for impl_16 (impl)
macro_rules! Depcrate_xxhash32impl_16 {
() => {
// Module: crate::xxhash32
// Provides: {"impl_16"}
// Dependencies: {}
impl BufferData { const fn new () -> Self { Self ([0 ; 4]) } const fn bytes (& self) -> & Bytes { const _ : () = assert ! (mem :: align_of ::< u8 > () <= mem :: align_of ::< Lane > ()) ; unsafe { & * self . 0 . as_ptr () . cast () } } fn bytes_mut (& mut self) -> & mut Bytes { unsafe { & mut * self . 0 . as_mut_ptr () . cast () } } }
};
}
