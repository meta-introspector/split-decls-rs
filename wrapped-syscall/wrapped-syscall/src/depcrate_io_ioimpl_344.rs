// Generated macro for impl_344 (impl)
macro_rules! Depcrate_io_ioimpl_344 {
() => {
// Module: crate::io::io
// Provides: {"impl_344"}
// Dependencies: {}
impl < I : Io > WriteOnly < I > { # [inline (always)] pub fn write (& mut self , value : I :: Value) { self . inner . write (value) } # [inline (always)] pub fn writef (& mut self , flags : I :: Value , value : bool) { self . inner . writef (flags , value) } }
};
}
