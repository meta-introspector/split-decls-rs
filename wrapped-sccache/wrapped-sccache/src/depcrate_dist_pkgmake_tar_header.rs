// Generated macro for make_tar_header (function)
macro_rules! Depcrate_dist_pkgmake_tar_header {
() => {
// Module: crate::dist::pkg
// Provides: {"make_tar_header"}
// Dependencies: {}
pub fn make_tar_header (src : & Path , dest : & str) -> io :: Result < tar :: Header > { let metadata_res = fs :: metadata (src) ; let mut file_header = tar :: Header :: new_ustar () ; if let Ok (metadata) = metadata_res { file_header . set_metadata (& metadata) ; } else { warn ! ("Couldn't get metadata of file {:?}, falling back to some defaults" , src) ; file_header . set_mode (0o644) ; file_header . set_uid (0) ; file_header . set_gid (0) ; file_header . set_mtime (0) ; file_header . set_device_major (0) . expect ("expected a ustar header") ; file_header . set_device_minor (0) . expect ("expected a ustar header") ; file_header . set_entry_type (tar :: EntryType :: file ()) ; } assert ! (dest . starts_with ('/')) ; let dest = dest . trim_start_matches ('/') ; assert ! (! dest . starts_with ('/')) ; file_header . set_path (dest) ? ; Ok (file_header) }
};
}
