// Generated macro for get_group_name (function)
macro_rules! Depcrate_unix_usersget_group_name {
() => {
// Module: crate::unix::users
// Provides: {"get_group_name"}
// Dependencies: {}
pub (crate) unsafe fn get_group_name (id : libc :: gid_t , buffer : & mut Vec < libc :: c_char > ,) -> Option < String > { let mut g = std :: mem :: MaybeUninit :: < libc :: group > :: uninit () ; let mut tmp_ptr = std :: ptr :: null_mut () ; let mut last_errno = 0 ; unsafe { loop { if retry_eintr ! (set_to_0 => last_errno => getgrgid_r (id as _ , g . as_mut_ptr () as _ , buffer . as_mut_ptr () , buffer . capacity () as _ , & mut tmp_ptr as _)) != 0 { if last_errno == libc :: ERANGE as libc :: c_int { buffer . set_len (buffer . capacity ()) ; buffer . reserve (2048) ; continue ; } return None ; } break ; } let g = g . assume_init () ; super :: utils :: cstr_to_rust (g . gr_name) } }
};
}
