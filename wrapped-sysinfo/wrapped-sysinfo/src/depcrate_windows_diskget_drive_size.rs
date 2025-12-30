// Generated macro for get_drive_size (function)
macro_rules! Depcrate_windows_diskget_drive_size {
() => {
// Module: crate::windows::disk
// Provides: {"get_drive_size"}
// Dependencies: {}
unsafe fn get_drive_size (mount_point : & [u16]) -> Option < (u64 , u64) > { let mut total_size = 0 ; let mut available_space = 0 ; let lpdirectoryname = PCWSTR :: from_raw (mount_point . as_ptr ()) ; if unsafe { GetDiskFreeSpaceExW (lpdirectoryname , None , Some (& mut total_size) , Some (& mut available_space) ,) } . is_ok () { Some ((total_size , available_space)) } else { None } }
};
}
