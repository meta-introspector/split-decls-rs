// Generated macro for impl_2971 (impl)
macro_rules! Depcrate_processimpl_2971 {
() => {
// Module: crate::process
// Provides: {"impl_2971"}
// Dependencies: {}
# [stable (feature = "process" , since = "1.0.0")] impl Read for ChildStderr { fn read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { self . inner . read (buf) } fn read_buf (& mut self , buf : BorrowedCursor < '_ >) -> io :: Result < () > { self . inner . read_buf (buf) } fn read_vectored (& mut self , bufs : & mut [IoSliceMut < '_ >]) -> io :: Result < usize > { self . inner . read_vectored (bufs) } # [inline] fn is_read_vectored (& self) -> bool { self . inner . is_read_vectored () } fn read_to_end (& mut self , buf : & mut Vec < u8 >) -> io :: Result < usize > { self . inner . read_to_end (buf) } }
};
}
