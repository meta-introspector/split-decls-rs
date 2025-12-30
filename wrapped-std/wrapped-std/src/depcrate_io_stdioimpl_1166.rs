// Generated macro for impl_1166 (impl)
macro_rules! Depcrate_io_stdioimpl_1166 {
() => {
// Module: crate::io::stdio
// Provides: {"impl_1166"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl Write for StdoutLock < '_ > { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { self . inner . borrow_mut () . write (buf) } fn write_vectored (& mut self , bufs : & [IoSlice < '_ >]) -> io :: Result < usize > { self . inner . borrow_mut () . write_vectored (bufs) } # [inline] fn is_write_vectored (& self) -> bool { self . inner . borrow_mut () . is_write_vectored () } fn flush (& mut self) -> io :: Result < () > { self . inner . borrow_mut () . flush () } fn write_all (& mut self , buf : & [u8]) -> io :: Result < () > { self . inner . borrow_mut () . write_all (buf) } fn write_all_vectored (& mut self , bufs : & mut [IoSlice < '_ >]) -> io :: Result < () > { self . inner . borrow_mut () . write_all_vectored (bufs) } }
};
}
