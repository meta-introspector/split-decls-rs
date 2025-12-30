// Generated macro for impl_157 (impl)
macro_rules! Depcrate_tendrilimpl_157 {
() => {
// Module: crate::tendril
// Provides: {"impl_157"}
// Dependencies: {}
impl < A > io :: Write for Tendril < fmt :: Bytes , A > where A : Atomicity , { # [inline] fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { self . push_slice (buf) ; Ok (buf . len ()) } # [inline] fn write_all (& mut self , buf : & [u8]) -> io :: Result < () > { self . push_slice (buf) ; Ok (()) } # [inline (always)] fn flush (& mut self) -> io :: Result < () > { Ok (()) } }
};
}
