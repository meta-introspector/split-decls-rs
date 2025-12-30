// Generated macro for load_statvfs_values (function)
macro_rules! Depcrate_unix_linux_diskload_statvfs_values {
() => {
// Module: crate::unix::linux::disk
// Provides: {"load_statvfs_values"}
// Dependencies: {}
unsafe fn load_statvfs_values (mount_point : & Path) -> Option < (u64 , u64 , bool) > { let mount_point_cpath = to_cpath (mount_point) ; let mut stat : MaybeUninit < statvfs > = MaybeUninit :: uninit () ; if unsafe { retry_eintr ! (statvfs (mount_point_cpath . as_ptr () as * const _ , stat . as_mut_ptr ())) } == 0 { let stat = unsafe { stat . assume_init () } ; let bsize = cast ! (stat . f_bsize) ; let blocks = cast ! (stat . f_blocks) ; let bavail = cast ! (stat . f_bavail) ; let total = bsize . saturating_mul (blocks) ; if total == 0 { return None ; } let available = bsize . saturating_mul (bavail) ; let is_read_only = (stat . f_flag & libc :: ST_RDONLY) != 0 ; Some ((total , available , is_read_only)) } else { None } }
};
}
