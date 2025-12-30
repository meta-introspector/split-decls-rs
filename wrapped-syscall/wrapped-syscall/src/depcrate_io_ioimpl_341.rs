// Generated macro for impl_341 (impl)
macro_rules! Depcrate_io_ioimpl_341 {
() => {
// Module: crate::io::io
// Provides: {"impl_341"}
// Dependencies: {}
impl < I : Io > ReadOnly < I > { # [inline (always)] pub fn read (& self) -> I :: Value { self . inner . read () } # [inline (always)] pub fn readf (& self , flags : I :: Value) -> bool { self . inner . readf (flags) } }
};
}
