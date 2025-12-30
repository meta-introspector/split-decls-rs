// Generated macro for impl_93 (impl)
macro_rules! Depcrate_fileimpl_93 {
() => {
// Module: crate::file
// Provides: {"impl_93"}
// Dependencies: {}
impl < F : Read > Read for NamedTempFile < F > { fn read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { self . as_file_mut () . read (buf) . with_err_path (| | self . path ()) } fn read_vectored (& mut self , bufs : & mut [io :: IoSliceMut < '_ >]) -> io :: Result < usize > { self . as_file_mut () . read_vectored (bufs) . with_err_path (| | self . path ()) } fn read_to_end (& mut self , buf : & mut Vec < u8 >) -> io :: Result < usize > { self . as_file_mut () . read_to_end (buf) . with_err_path (| | self . path ()) } fn read_to_string (& mut self , buf : & mut String) -> io :: Result < usize > { self . as_file_mut () . read_to_string (buf) . with_err_path (| | self . path ()) } fn read_exact (& mut self , buf : & mut [u8]) -> io :: Result < () > { self . as_file_mut () . read_exact (buf) . with_err_path (| | self . path ()) } }
};
}
