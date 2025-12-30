// Generated macro for get_volume_path_names_for_volume_name (function)
macro_rules! Depcrate_windows_diskget_volume_path_names_for_volume_name {
() => {
// Module: crate::windows::disk
// Provides: {"get_volume_path_names_for_volume_name"}
// Dependencies: {}
# [doc = " Given a volume GUID path (`\\\\?\\{xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx}\\`), returns all"] # [doc = " volume paths (drive letters and mount paths) associated with it"] # [doc = " as zero terminated wide strings."] # [doc = ""] # [doc = " # Safety"] # [doc = " `volume_name` must contain a zero-terminated wide string."] pub (crate) unsafe fn get_volume_path_names_for_volume_name (volume_guid_path : & [u16] ,) -> Vec < Vec < u16 > > { let volume_guid_path = PCWSTR :: from_raw (volume_guid_path . as_ptr ()) ; let mut path_names_buf = vec ! [0u16 ; MAX_PATH as usize] ; let mut path_names_output_size = 0u32 ; for _ in 0 .. 10 { let volume_path_names = unsafe { GetVolumePathNamesForVolumeNameW (volume_guid_path , Some (path_names_buf . as_mut_slice ()) , & mut path_names_output_size ,) } ; let code = volume_path_names . map_err (| _ | Error :: from_thread () . code ()) ; match code { Ok (()) => break , Err (ERROR_MORE_DATA) => { path_names_buf = vec ! [0u16 ; path_names_output_size as usize] ; continue ; } Err (_e) => { sysinfo_debug ! ("Error: GetVolumePathNamesForVolumeNameW() = {}" , _e) ; return Vec :: new () ; } } } let mut path_names = Vec :: new () ; let mut buf = & path_names_buf [..] ; while ! buf . is_empty () && buf [0] != 0 { let path = from_zero_terminated (buf) ; buf = & buf [path . len () ..] ; path_names . push (path) ; } path_names }
};
}
