// Generated macro for impl_97 (impl)
macro_rules! Depcrate_fileimpl_97 {
() => {
// Module: crate::file
// Provides: {"impl_97"}
// Dependencies: {}
impl < F : Seek > Seek for NamedTempFile < F > { fn seek (& mut self , pos : SeekFrom) -> io :: Result < u64 > { self . as_file_mut () . seek (pos) . with_err_path (| | self . path ()) } }
};
}
