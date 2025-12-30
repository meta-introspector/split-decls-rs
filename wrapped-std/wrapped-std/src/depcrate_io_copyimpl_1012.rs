// Generated macro for impl_1012 (impl)
macro_rules! Depcrate_io_copyimpl_1012 {
() => {
// Module: crate::io::copy
// Provides: {"impl_1012"}
// Dependencies: {}
impl < T > BufferedReaderSpec for T where Self : Read , T : ? Sized , { # [inline] default fn buffer_size (& self) -> usize { 0 } default fn copy_to (& mut self , _to : & mut (impl Write + ? Sized)) -> Result < u64 > { unreachable ! ("only called from specializations") } }
};
}
