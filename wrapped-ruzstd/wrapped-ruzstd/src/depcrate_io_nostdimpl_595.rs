// Generated macro for impl_595 (impl)
macro_rules! Depcrate_io_nostdimpl_595 {
() => {
// Module: crate::io_nostd
// Provides: {"impl_595"}
// Dependencies: {}
impl Write for alloc :: vec :: Vec < u8 > { # [inline] fn write (& mut self , data : & [u8]) -> Result < usize , Error > { self . extend_from_slice (data) ; Ok (data . len ()) } fn flush (& mut self) -> Result < () , Error > { Ok (()) } }
};
}
