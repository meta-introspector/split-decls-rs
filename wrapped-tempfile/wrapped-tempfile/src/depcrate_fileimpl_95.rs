// Generated macro for impl_95 (impl)
macro_rules! Depcrate_fileimpl_95 {
() => {
// Module: crate::file
// Provides: {"impl_95"}
// Dependencies: {}
impl < F : Write > Write for NamedTempFile < F > { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { self . as_file_mut () . write (buf) . with_err_path (| | self . path ()) } # [inline] fn flush (& mut self) -> io :: Result < () > { self . as_file_mut () . flush () . with_err_path (| | self . path ()) } fn write_vectored (& mut self , bufs : & [io :: IoSlice < '_ >]) -> io :: Result < usize > { self . as_file_mut () . write_vectored (bufs) . with_err_path (| | self . path ()) } fn write_all (& mut self , buf : & [u8]) -> io :: Result < () > { self . as_file_mut () . write_all (buf) . with_err_path (| | self . path ()) } fn write_fmt (& mut self , fmt : fmt :: Arguments < '_ >) -> io :: Result < () > { self . as_file_mut () . write_fmt (fmt) . with_err_path (| | self . path ()) } }
};
}
