// Generated macro for get_users (function)
macro_rules! Depcrate_unix_usersget_users {
() => {
// Module: crate::unix::users
// Provides: {"get_users"}
// Dependencies: {}
pub (crate) fn get_users (users : & mut Vec < User >) { fn filter (shell : * const std :: ffi :: c_char , uid : u32) -> bool { ! endswith (shell , b"/false") && ! endswith (shell , b"/uucico") && uid < 65536 } users . clear () ; let mut users_map = std :: collections :: HashMap :: with_capacity (10) ; unsafe { setpwent () ; loop { let pw = getpwent () ; if pw . is_null () { if std :: io :: Error :: last_os_error () . kind () == std :: io :: ErrorKind :: Interrupted { continue ; } break ; } if ! filter ((* pw) . pw_shell , (* pw) . pw_uid) { continue ; } if let Some (name) = crate :: unix :: utils :: cstr_to_rust ((* pw) . pw_name) { if users_map . contains_key (& name) { continue ; } let uid = (* pw) . pw_uid ; let gid = (* pw) . pw_gid ; users_map . insert (name , (Uid (uid) , Gid (gid))) ; } } endpwent () ; } for (name , (uid , gid)) in users_map { users . push (User { inner : UserInner :: new (uid , gid , name) , }) ; } }
};
}
