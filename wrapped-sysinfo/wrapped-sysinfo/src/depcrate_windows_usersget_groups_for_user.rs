// Generated macro for get_groups_for_user (function)
macro_rules! Depcrate_windows_usersget_groups_for_user {
() => {
// Module: crate::windows::users
// Provides: {"get_groups_for_user"}
// Dependencies: {}
# [doc = " Get the groups for a user."] # [doc = ""] # [doc = " # Safety"] # [doc = " The caller must ensure that the `username` is a valid wide Unicode string with a null terminator."] unsafe fn get_groups_for_user (username : PCWSTR) -> Vec < Group > { let mut buf : NetApiBuffer < LOCALGROUP_USERS_INFO_0 > = Default :: default () ; let mut nb_entries = 0 ; let mut total_entries = 0 ; let mut groups : Vec < Group > ; let status = unsafe { NetUserGetLocalGroups (PCWSTR :: null () , username , 0 , LG_INCLUDE_INDIRECT , buf . inner_mut_as_bytes () , MAX_PREFERRED_LENGTH , & mut nb_entries , & mut total_entries ,) } ; if status == NERR_Success { groups = Vec :: with_capacity (nb_entries as _) ; if ! buf . 0 . is_null () { unsafe { let entries = std :: slice :: from_raw_parts (buf . 0 , nb_entries as _) ; groups . extend (entries . iter () . map (| entry | Group { inner : GroupInner :: new (Gid (0) , to_utf8_str (entry . lgrui0_name)) , })) ; } } } else { groups = Vec :: new () ; sysinfo_debug ! ("NetUserGetLocalGroups failed with ret code {}" , status) ; } groups }
};
}
