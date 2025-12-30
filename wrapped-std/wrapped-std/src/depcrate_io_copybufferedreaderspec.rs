// Generated macro for BufferedReaderSpec (trait)
macro_rules! Depcrate_io_copyBufferedReaderSpec {
() => {
// Module: crate::io::copy
// Provides: {"BufferedReaderSpec"}
// Dependencies: {}
# [doc = " Specialization of the read-write loop that reuses the internal"] # [doc = " buffer of a BufReader. If there's no buffer then the writer side"] # [doc = " should be used instead."] trait BufferedReaderSpec { fn buffer_size (& self) -> usize ; fn copy_to (& mut self , to : & mut (impl Write + ? Sized)) -> Result < u64 > ; }
};
}
