// Generated macro for get_groups (function)
macro_rules! Depcrate_windows_groupsget_groups {
() => {
// Module: crate::windows::groups
// Provides: {"get_groups"}
// Dependencies: {}
pub (crate) fn get_groups (groups : & mut Vec < Group >) { groups . clear () ; unsafe { let mut nb_entries = 0 ; let mut total_entries_hint = 0 ; let mut handle = 0 ; loop { let mut buff = NetApiBuffer :: default () ; let res = NetLocalGroupEnum (None , 0 , buff . inner_mut () as * mut _ , MAX_PREFERRED_LENGTH , & mut nb_entries , & mut total_entries_hint , Some (& mut handle) ,) ; if res != ERROR_SUCCESS . 0 && res != ERROR_MORE_DATA . 0 { sysinfo_debug ! ("NetLocalGroupEnum failed: {res:?}") ; break ; } let entries = std :: slice :: from_raw_parts (buff . 0 , nb_entries as usize) ; for entry in entries { let name = to_utf8_str (entry . lgrpi0_name) ; groups . push (Group { inner : GroupInner :: new (Gid (0) , name) , }) ; } if res != ERROR_MORE_DATA . 0 { break ; } } } }
};
}
