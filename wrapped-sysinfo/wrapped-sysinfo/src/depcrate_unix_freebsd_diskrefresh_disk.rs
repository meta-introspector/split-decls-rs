// Generated macro for refresh_disk (function)
macro_rules! Depcrate_unix_freebsd_diskrefresh_disk {
() => {
// Module: crate::unix::freebsd::disk
// Provides: {"refresh_disk"}
// Dependencies: {}
fn refresh_disk (disk : & mut DiskInner , refresh_kind : DiskRefreshKind) -> bool { if refresh_kind . storage () { unsafe { let mut vfs : libc :: statvfs = std :: mem :: zeroed () ; if let Some ((total_space , available_space , is_read_only)) = get_statvfs (& disk . c_mount_point , & mut vfs) { disk . total_space = total_space ; disk . available_space = available_space ; disk . is_read_only = is_read_only ; } } } if refresh_kind . io_usage () { unsafe { refresh_disk_io (& mut [disk]) ; } } true }
};
}
