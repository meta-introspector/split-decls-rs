// Generated macro for write_central_directory_header (function)
macro_rules! Depcrate_writewrite_central_directory_header {
() => {
// Module: crate::write
// Provides: {"write_central_directory_header"}
// Dependencies: {}
fn write_central_directory_header < T : Write > (writer : & mut T , file : & ZipFileData) -> ZipResult < () > { let block = file . block () ? ; block . write (writer) ? ; writer . write_all (& file . file_name_raw) ? ; if let Some (extra_field) = & file . extra_field { writer . write_all (extra_field) ? ; } if let Some (central_extra_field) = & file . central_extra_field { writer . write_all (central_extra_field) ? ; } writer . write_all (file . file_comment . as_bytes ()) ? ; Ok (()) }
};
}
