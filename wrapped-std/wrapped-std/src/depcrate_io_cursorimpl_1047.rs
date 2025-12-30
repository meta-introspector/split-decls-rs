// Generated macro for impl_1047 (impl)
macro_rules! Depcrate_io_cursorimpl_1047 {
() => {
// Module: crate::io::cursor
// Provides: {"impl_1047"}
// Dependencies: {}
# [stable (feature = "cursor_array" , since = "1.61.0")] impl < const N : usize > Write for Cursor < [u8 ; N] > { # [inline] fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { slice_write (& mut self . pos , & mut self . inner , buf) } # [inline] fn write_vectored (& mut self , bufs : & [IoSlice < '_ >]) -> io :: Result < usize > { slice_write_vectored (& mut self . pos , & mut self . inner , bufs) } # [inline] fn is_write_vectored (& self) -> bool { true } # [inline] fn write_all (& mut self , buf : & [u8]) -> io :: Result < () > { slice_write_all (& mut self . pos , & mut self . inner , buf) } # [inline] fn write_all_vectored (& mut self , bufs : & mut [IoSlice < '_ >]) -> io :: Result < () > { slice_write_all_vectored (& mut self . pos , & mut self . inner , bufs) } # [inline] fn flush (& mut self) -> io :: Result < () > { Ok (()) } }
};
}
