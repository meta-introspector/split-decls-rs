// Generated macro for impl_94 (impl)
macro_rules! Depcrate_fileimpl_94 {
() => {
// Module: crate::file
// Provides: {"impl_94"}
// Dependencies: {}
impl Read for & NamedTempFile < File > { fn read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { self . as_file () . read (buf) . with_err_path (| | self . path ()) } fn read_vectored (& mut self , bufs : & mut [io :: IoSliceMut < '_ >]) -> io :: Result < usize > { self . as_file () . read_vectored (bufs) . with_err_path (| | self . path ()) } fn read_to_end (& mut self , buf : & mut Vec < u8 >) -> io :: Result < usize > { self . as_file () . read_to_end (buf) . with_err_path (| | self . path ()) } fn read_to_string (& mut self , buf : & mut String) -> io :: Result < usize > { self . as_file () . read_to_string (buf) . with_err_path (| | self . path ()) } fn read_exact (& mut self , buf : & mut [u8]) -> io :: Result < () > { self . as_file () . read_exact (buf) . with_err_path (| | self . path ()) } }
};
}
