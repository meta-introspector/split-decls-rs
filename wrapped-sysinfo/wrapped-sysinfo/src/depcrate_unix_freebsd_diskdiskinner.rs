// Generated macro for DiskInner (struct)
macro_rules! Depcrate_unix_freebsd_diskDiskInner {
() => {
// Module: crate::unix::freebsd::disk
// Provides: {"DiskInner"}
// Dependencies: {}
# [derive (Debug)] pub (crate) struct DiskInner { name : OsString , c_mount_point : Vec < libc :: c_char > , dev_id : Option < String > , mount_point : PathBuf , total_space : u64 , available_space : u64 , file_system : OsString , is_removable : bool , is_read_only : bool , read_bytes : u64 , old_read_bytes : u64 , written_bytes : u64 , old_written_bytes : u64 , updated : bool , }
};
}
