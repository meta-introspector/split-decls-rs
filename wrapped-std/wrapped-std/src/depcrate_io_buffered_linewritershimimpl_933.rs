// Generated macro for impl_933 (impl)
macro_rules! Depcrate_io_buffered_linewritershimimpl_933 {
() => {
// Module: crate::io::buffered::linewritershim
// Provides: {"impl_933"}
// Dependencies: {}
impl < 'a , W : ? Sized + Write > LineWriterShim < 'a , W > { pub fn new (buffer : & 'a mut BufWriter < W >) -> Self { Self { buffer } } # [doc = " Gets a reference to the inner writer (that is, the writer"] # [doc = " wrapped by the BufWriter)."] fn inner (& self) -> & W { self . buffer . get_ref () } # [doc = " Gets a mutable reference to the inner writer (that is, the writer"] # [doc = " wrapped by the BufWriter). Be careful with this writer, as writes to"] # [doc = " it will bypass the buffer."] fn inner_mut (& mut self) -> & mut W { self . buffer . get_mut () } # [doc = " Gets the content currently buffered in self.buffer"] fn buffered (& self) -> & [u8] { self . buffer . buffer () } # [doc = " Flushes the buffer iff the last byte is a newline (indicating that an"] # [doc = " earlier write only succeeded partially, and we want to retry flushing"] # [doc = " the buffered line before continuing with a subsequent write)."] fn flush_if_completed_line (& mut self) -> io :: Result < () > { match self . buffered () . last () . copied () { Some (b'\n') => self . buffer . flush_buf () , _ => Ok (()) , } } }
};
}
