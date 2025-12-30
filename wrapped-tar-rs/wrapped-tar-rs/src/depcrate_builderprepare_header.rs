// Generated macro for prepare_header (function)
macro_rules! Depcrate_builderprepare_header {
() => {
// Module: crate::builder
// Provides: {"prepare_header"}
// Dependencies: {}
fn prepare_header (size : u64 , entry_type : u8) -> Header { let mut header = Header :: new_gnu () ; let name = b"././@LongLink" ; header . as_gnu_mut () . unwrap () . name [.. name . len ()] . clone_from_slice (& name [..]) ; header . set_mode (0o644) ; header . set_uid (0) ; header . set_gid (0) ; header . set_mtime (0) ; header . set_size (size + 1) ; header . set_entry_type (EntryType :: new (entry_type)) ; header . set_cksum () ; header }
};
}
