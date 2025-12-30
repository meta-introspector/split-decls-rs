// Generated macro for impl_98 (impl)
macro_rules! Depcrate_fileimpl_98 {
() => {
// Module: crate::file
// Provides: {"impl_98"}
// Dependencies: {}
impl Seek for & NamedTempFile < File > { fn seek (& mut self , pos : SeekFrom) -> io :: Result < u64 > { self . as_file () . seek (pos) . with_err_path (| | self . path ()) } }
};
}
