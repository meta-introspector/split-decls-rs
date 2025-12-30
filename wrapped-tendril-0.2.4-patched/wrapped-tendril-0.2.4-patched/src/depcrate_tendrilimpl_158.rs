// Generated macro for impl_158 (impl)
macro_rules! Depcrate_tendrilimpl_158 {
() => {
// Module: crate::tendril
// Provides: {"impl_158"}
// Dependencies: {}
impl < A > encoding :: ByteWriter for Tendril < fmt :: Bytes , A > where A : Atomicity , { # [inline] fn write_byte (& mut self , b : u8) { self . push_slice (& [b]) ; } # [inline] fn write_bytes (& mut self , v : & [u8]) { self . push_slice (v) ; } # [inline] fn writer_hint (& mut self , additional : usize) { self . reserve (cmp :: min (u32 :: MAX as usize , additional) as u32) ; } }
};
}
