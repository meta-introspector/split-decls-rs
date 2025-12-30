// Generated macro for get_user_groups (function)
macro_rules! Depcrate_unix_usersget_user_groups {
() => {
// Module: crate::unix::users
// Provides: {"get_user_groups"}
// Dependencies: {}
pub (crate) unsafe fn get_user_groups (name : * const libc :: c_char , group_id : libc :: gid_t ,) -> Vec < Group > { let mut buffer = Vec :: with_capacity (2048) ; let mut groups = Vec :: with_capacity (256) ; loop { unsafe { let mut nb_groups = groups . capacity () ; if getgrouplist (name , group_id as _ , groups . as_mut_ptr () , & mut nb_groups as * mut _ as * mut _ ,) == - 1 { groups . set_len (nb_groups as _) ; groups . reserve (256) ; continue ; } groups . set_len (nb_groups as _) ; return groups . iter () . filter_map (| group_id | { let name = get_group_name (* group_id as _ , & mut buffer) ? ; Some (Group { inner : crate :: GroupInner :: new (Gid (* group_id as _) , name) , }) }) . collect () ; } } }
};
}
