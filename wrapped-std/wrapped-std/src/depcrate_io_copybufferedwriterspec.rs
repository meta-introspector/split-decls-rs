// Generated macro for BufferedWriterSpec (trait)
macro_rules! Depcrate_io_copyBufferedWriterSpec {
() => {
// Module: crate::io::copy
// Provides: {"BufferedWriterSpec"}
// Dependencies: {}
# [doc = " Specialization of the read-write loop that either uses a stack buffer"] # [doc = " or reuses the internal buffer of a BufWriter"] trait BufferedWriterSpec : Write { fn buffer_size (& self) -> usize ; fn copy_from < R : Read + ? Sized > (& mut self , reader : & mut R) -> Result < u64 > ; }
};
}
