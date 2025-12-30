// Generated macro for impl_1043 (impl)
macro_rules! Depcrate_io_cursorimpl_1043 {
() => {
// Module: crate::io::cursor
// Provides: {"impl_1043"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl Write for Cursor < & mut [u8] > { # [inline] fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { slice_write (& mut self . pos , self . inner , buf) } # [inline] fn write_vectored (& mut self , bufs : & [IoSlice < '_ >]) -> io :: Result < usize > { slice_write_vectored (& mut self . pos , self . inner , bufs) } # [inline] fn is_write_vectored (& self) -> bool { true } # [inline] fn write_all (& mut self , buf : & [u8]) -> io :: Result < () > { slice_write_all (& mut self . pos , self . inner , buf) } # [inline] fn write_all_vectored (& mut self , bufs : & mut [IoSlice < '_ >]) -> io :: Result < () > { slice_write_all_vectored (& mut self . pos , self . inner , bufs) } # [inline] fn flush (& mut self) -> io :: Result < () > { Ok (()) } }
};
}
