// Generated macro for impl_1013 (impl)
macro_rules! Depcrate_io_copyimpl_1013 {
() => {
// Module: crate::io::copy
// Provides: {"impl_1013"}
// Dependencies: {}
impl BufferedReaderSpec for & [u8] { fn buffer_size (& self) -> usize { usize :: MAX } fn copy_to (& mut self , to : & mut (impl Write + ? Sized)) -> Result < u64 > { let len = self . len () ; to . write_all (self) ? ; * self = & self [len ..] ; Ok (len as u64) } }
};
}
