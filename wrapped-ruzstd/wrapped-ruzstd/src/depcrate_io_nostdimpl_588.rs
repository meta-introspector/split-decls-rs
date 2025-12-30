// Generated macro for impl_588 (impl)
macro_rules! Depcrate_io_nostdimpl_588 {
() => {
// Module: crate::io_nostd
// Provides: {"impl_588"}
// Dependencies: {}
impl < T > Read for & mut T where T : Read , { fn read (& mut self , buf : & mut [u8]) -> Result < usize , Error > { (* self) . read (buf) } }
};
}
