// Generated macro for impl_163 (impl)
macro_rules! Depcrate_tendrilimpl_163 {
() => {
// Module: crate::tendril
// Provides: {"impl_163"}
// Dependencies: {}
impl < A > encoding :: StringWriter for Tendril < fmt :: UTF8 , A > where A : Atomicity , { # [inline] fn write_char (& mut self , c : char) { self . push_char (c) ; } # [inline] fn write_str (& mut self , s : & str) { self . push_slice (s) ; } # [inline] fn writer_hint (& mut self , additional : usize) { self . reserve (cmp :: min (u32 :: MAX as usize , additional) as u32) ; } }
};
}
