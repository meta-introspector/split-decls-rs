// Generated macro for file_size (function)
macro_rules! Depcrate_gzfile_size {
() => {
// Module: crate::gz
// Provides: {"file_size"}
// Dependencies: {}
fn file_size (path : & str) -> Result < usize , () > { let mut result = Err (()) ; let stat_ptr = unsafe { libc :: calloc (1 , core :: mem :: size_of :: < libc :: stat > () as _) } ; if stat_ptr . is_null () { return result ; } let ret = unsafe { libc :: stat (CString :: new (path) . unwrap () . as_ptr () , stat_ptr . cast :: < libc :: stat > () ,) } ; if ret == 0 { if let Some (stat_info) = unsafe { stat_ptr . cast :: < libc :: stat > () . as_ref () } { result = Ok (stat_info . st_size as usize) ; } } unsafe { libc :: free (stat_ptr) } ; result }
};
}
