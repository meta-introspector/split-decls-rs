// Generated macro for DiskInner (struct)
macro_rules! Depcrate_windows_diskDiskInner {
() => {
// Module: crate::windows::disk
// Provides: {"DiskInner"}
// Dependencies: {}
pub (crate) struct DiskInner { type_ : DiskKind , name : OsString , file_system : OsString , mount_point : Vec < u16 > , s_mount_point : OsString , total_space : u64 , available_space : u64 , is_removable : bool , is_read_only : bool , device_path : Vec < u16 > , old_written_bytes : u64 , old_read_bytes : u64 , written_bytes : u64 , read_bytes : u64 , updated : bool , }
};
}
