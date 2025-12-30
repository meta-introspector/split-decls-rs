// Generated macro for append_fs (function)
macro_rules! Depcrate_builderappend_fs {
() => {
// Module: crate::builder
// Provides: {"append_fs"}
// Dependencies: {}
fn append_fs (dst : & mut dyn Write , path : & Path , meta : & fs :: Metadata , mode : HeaderMode , link_name : Option < & Path > ,) -> io :: Result < () > { let mut header = Header :: new_gnu () ; prepare_header_path (dst , & mut header , path) ? ; header . set_metadata_in_mode (meta , mode) ; if let Some (link_name) = link_name { prepare_header_link (dst , & mut header , link_name) ? ; } header . set_cksum () ; dst . write_all (header . as_bytes ()) }
};
}
