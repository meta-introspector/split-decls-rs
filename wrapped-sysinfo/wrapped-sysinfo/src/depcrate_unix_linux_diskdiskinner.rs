// Generated macro for DiskInner (struct)
macro_rules! Depcrate_unix_linux_diskDiskInner {
() => {
// Module: crate::unix::linux::disk
// Provides: {"DiskInner"}
// Dependencies: {}
pub (crate) struct DiskInner { type_ : DiskKind , device_name : OsString , actual_device_name : Option < String > , file_system : OsString , mount_point : PathBuf , total_space : u64 , available_space : u64 , is_removable : bool , is_read_only : bool , old_written_bytes : u64 , old_read_bytes : u64 , written_bytes : u64 , read_bytes : u64 , updated : bool , }
};
}
