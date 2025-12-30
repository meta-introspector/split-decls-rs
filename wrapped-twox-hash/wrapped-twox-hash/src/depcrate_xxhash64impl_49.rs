// Generated macro for impl_49 (impl)
macro_rules! Depcrate_xxhash64impl_49 {
() => {
// Module: crate::xxhash64
// Provides: {"impl_49"}
// Dependencies: {}
impl BufferData { const fn new () -> Self { Self ([0 ; 4]) } const fn bytes (& self) -> & Bytes { const _ : () = assert ! (mem :: align_of ::< u8 > () <= mem :: align_of ::< Lane > ()) ; unsafe { & * self . 0 . as_ptr () . cast () } } fn bytes_mut (& mut self) -> & mut Bytes { unsafe { & mut * self . 0 . as_mut_ptr () . cast () } } }
};
}
