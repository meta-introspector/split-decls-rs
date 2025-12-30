// Generated macro for get_volume_guid_paths (function)
macro_rules! Depcrate_windows_diskget_volume_guid_paths {
() => {
// Module: crate::windows::disk
// Provides: {"get_volume_guid_paths"}
// Dependencies: {}
# [doc = " Returns a list of zero-terminated wide strings containing volume GUID paths."] # [doc = " Volume GUID paths have the form `\\\\?\\{xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx}\\`."] # [doc = ""] # [doc = " Rather confusingly, the Win32 API _also_ calls these \"volume names\"."] pub (crate) fn get_volume_guid_paths () -> Vec < Vec < u16 > > { let mut volume_names = Vec :: new () ; unsafe { let mut buf = Box :: new ([0u16 ; VOLUME_NAME_SIZE]) ; let Ok (handle) = FindFirstVolumeW (& mut buf [..]) else { sysinfo_debug ! ("Error: FindFirstVolumeW() = {:?}" , Error :: from_thread () . code ()) ; return Vec :: new () ; } ; volume_names . push (from_zero_terminated (& buf [..])) ; loop { if FindNextVolumeW (handle , & mut buf [..]) . is_err () { if Error :: from_thread () . code () != ERROR_NO_MORE_FILES { sysinfo_debug ! ("Error: FindNextVolumeW = {}" , Error :: from_thread () . code ()) ; } break ; } volume_names . push (from_zero_terminated (& buf [..])) ; } if FindVolumeClose (handle) . is_err () { sysinfo_debug ! ("Error: FindVolumeClose = {:?}" , Error :: from_thread () . code ()) ; } ; } volume_names }
};
}
