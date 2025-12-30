// Generated macro for FileOpened (struct)
macro_rules! Depcrate_services_fs_serve_dir_open_fileFileOpened {
() => {
// Module: crate::services::fs::serve_dir::open_file
// Provides: {"FileOpened"}
// Dependencies: {}
pub (super) struct FileOpened { pub (super) extent : FileRequestExtent , pub (super) chunk_size : usize , pub (super) mime_header_value : HeaderValue , pub (super) maybe_encoding : Option < Encoding > , pub (super) maybe_range : Option < Result < Vec < RangeInclusive < u64 > > , RangeUnsatisfiableError > > , pub (super) last_modified : Option < LastModified > , }
};
}
