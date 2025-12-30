// Generated macro for append_file (function)
macro_rules! Depcrate_builderappend_file {
() => {
// Module: crate::builder
// Provides: {"append_file"}
// Dependencies: {}
fn append_file (dst : & mut dyn Write , path : & Path , file : & mut fs :: File , options : BuilderOptions ,) -> io :: Result < () > { let stat = file . metadata () ? ; let mut header = Header :: new_gnu () ; prepare_header_path (dst , & mut header , path) ? ; header . set_metadata_in_mode (& stat , options . mode) ; let sparse_entries = if options . sparse { prepare_header_sparse (file , & stat , & mut header) ? } else { None } ; header . set_cksum () ; dst . write_all (header . as_bytes ()) ? ; if let Some (sparse_entries) = sparse_entries { append_extended_sparse_headers (dst , & sparse_entries) ? ; for entry in sparse_entries . entries { file . seek (io :: SeekFrom :: Start (entry . offset)) ? ; io :: copy (& mut file . take (entry . num_bytes) , dst) ? ; } pad_zeroes (dst , sparse_entries . on_disk_size) ? ; } else { let len = io :: copy (file , dst) ? ; pad_zeroes (dst , len) ? ; } Ok (()) }
};
}
