// Generated macro for impl_1110 (impl)
macro_rules! Depcrate_io_pipeimpl_1110 {
() => {
// Module: crate::io::pipe
// Provides: {"impl_1110"}
// Dependencies: {}
# [stable (feature = "anonymous_pipe" , since = "1.87.0")] impl io :: Read for & PipeReader { fn read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { self . 0 . read (buf) } fn read_vectored (& mut self , bufs : & mut [io :: IoSliceMut < '_ >]) -> io :: Result < usize > { self . 0 . read_vectored (bufs) } # [inline] fn is_read_vectored (& self) -> bool { self . 0 . is_read_vectored () } fn read_to_end (& mut self , buf : & mut Vec < u8 >) -> io :: Result < usize > { self . 0 . read_to_end (buf) } fn read_buf (& mut self , buf : io :: BorrowedCursor < '_ >) -> io :: Result < () > { self . 0 . read_buf (buf) } }
};
}
