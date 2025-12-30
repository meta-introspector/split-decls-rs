// Generated macro for impl_953 (impl)
macro_rules! Depcrate_unix_usersimpl_953 {
() => {
// Module: crate::unix::users
// Provides: {"impl_953"}
// Dependencies: {}
impl UserInner { pub (crate) fn new (uid : Uid , gid : Gid , name : String) -> Self { let mut c_user = name . as_bytes () . to_vec () ; c_user . push (0) ; Self { uid , gid , name , c_user , } } pub (crate) fn id (& self) -> & Uid { & self . uid } pub (crate) fn group_id (& self) -> Gid { self . gid } pub (crate) fn name (& self) -> & str { & self . name } pub (crate) fn groups (& self) -> Vec < Group > { unsafe { get_user_groups (self . c_user . as_ptr () as * const _ , self . gid . 0 as _) } } }
};
}
