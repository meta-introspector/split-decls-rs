// Generated macro for impl_1017 (impl)
macro_rules! Depcrate_io_copyimpl_1017 {
() => {
// Module: crate::io::copy
// Provides: {"impl_1017"}
// Dependencies: {}
impl < W : Write + ? Sized > BufferedWriterSpec for W { # [inline] default fn buffer_size (& self) -> usize { 0 } default fn copy_from < R : Read + ? Sized > (& mut self , reader : & mut R) -> Result < u64 > { stack_buffer_copy (reader , self) } }
};
}
