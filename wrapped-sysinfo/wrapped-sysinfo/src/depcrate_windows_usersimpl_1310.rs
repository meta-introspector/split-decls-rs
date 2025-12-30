// Generated macro for impl_1310 (impl)
macro_rules! Depcrate_windows_usersimpl_1310 {
() => {
// Module: crate::windows::users
// Provides: {"impl_1310"}
// Dependencies: {}
impl UserInner { fn new (uid : Uid , name : String , c_name : PCWSTR , is_local : bool) -> Self { let c_user_name = if c_name . is_null () { None } else { Some (unsafe { c_name . as_wide () } . into ()) } ; Self { uid , gid : Gid (0) , name , c_user_name , is_local , } } pub (crate) fn id (& self) -> & Uid { & self . uid } pub (crate) fn group_id (& self) -> Gid { self . gid } pub (crate) fn name (& self) -> & str { & self . name } pub (crate) fn groups (& self) -> Vec < Group > { if let (Some (c_user_name) , true) = (& self . c_user_name , self . is_local) { let username = { let mut null_terminated = c_user_name . to_vec () ; if null_terminated . last () . is_some_and (| v | * v != 0) { null_terminated . push (0) ; } null_terminated } ; unsafe { get_groups_for_user (PCWSTR :: from_raw (username . as_ptr ())) } } else { Vec :: new () } } }
};
}
