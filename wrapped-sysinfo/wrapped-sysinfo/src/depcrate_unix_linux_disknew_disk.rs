// Generated macro for new_disk (function)
macro_rules! Depcrate_unix_linux_disknew_disk {
() => {
// Module: crate::unix::linux::disk
// Provides: {"new_disk"}
// Dependencies: {}
fn new_disk (device_name : & OsStr , mount_point : & Path , file_system : & OsStr , removable_entries : & [PathBuf] , procfs_disk_stats : & HashMap < String , DiskStat > , refresh_kind : DiskRefreshKind ,) -> Disk { let is_removable = removable_entries . iter () . any (| e | e . as_os_str () == device_name) ; let mut disk = Disk { inner : DiskInner { type_ : DiskKind :: Unknown (- 1) , device_name : device_name . to_owned () , actual_device_name : None , file_system : file_system . to_owned () , mount_point : mount_point . to_owned () , total_space : 0 , available_space : 0 , is_removable , is_read_only : false , old_read_bytes : 0 , old_written_bytes : 0 , read_bytes : 0 , written_bytes : 0 , updated : true , } , } ; disk . inner . efficient_refresh (refresh_kind , procfs_disk_stats , true) ; disk }
};
}
