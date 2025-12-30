// Generated macro for impl_1019 (impl)
macro_rules! Depcrate_io_copyimpl_1019 {
() => {
// Module: crate::io::copy
// Provides: {"impl_1019"}
// Dependencies: {}
impl BufferedWriterSpec for Vec < u8 > { fn buffer_size (& self) -> usize { cmp :: max (DEFAULT_BUF_SIZE , self . capacity () - self . len ()) } fn copy_from < R : Read + ? Sized > (& mut self , reader : & mut R) -> Result < u64 > { reader . read_to_end (self) . map (| bytes | u64 :: try_from (bytes) . expect ("usize overflowed u64")) } }
};
}
