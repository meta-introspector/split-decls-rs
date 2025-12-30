// Generated macro for impl_523 (impl)
macro_rules! Depcrate_io_nostdimpl_523 {
() => {
// Module: crate::io_nostd
// Provides: {"impl_523"}
// Dependencies: {}
impl < T > Write for & mut T where T : Write , { fn write (& mut self , buf : & [u8]) -> Result < usize , Error > { (* self) . write (buf) } fn flush (& mut self) -> Result < () , Error > { (* self) . flush () } }
};
}
