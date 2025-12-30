// Generated macro for get_statvfs (function)
macro_rules! Depcrate_unix_freebsd_diskget_statvfs {
() => {
// Module: crate::unix::freebsd::disk
// Provides: {"get_statvfs"}
// Dependencies: {}
# [doc = " Returns `(total_space, available_space, is_read_only)`."] unsafe fn get_statvfs (c_mount_point : & [libc :: c_char] , vfs : & mut libc :: statvfs ,) -> Option < (u64 , u64 , bool) > { if unsafe { libc :: statvfs (c_mount_point . as_ptr () as * const _ , vfs as * mut _) < 0 } { sysinfo_debug ! ("statvfs failed") ; None } else { let block_size : u64 = vfs . f_frsize as _ ; Some ((vfs . f_blocks . saturating_mul (block_size) , vfs . f_favail . saturating_mul (block_size) , (vfs . f_flag & libc :: ST_RDONLY) != 0 ,)) } }
};
}
