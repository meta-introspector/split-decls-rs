// Generated macro for DiskInner (struct)
macro_rules! Depcrate_unix_apple_diskDiskInner {
() => {
// Module: crate::unix::apple::disk
// Provides: {"DiskInner"}
// Dependencies: {}
pub (crate) struct DiskInner { pub (crate) type_ : DiskKind , pub (crate) name : OsString , # [cfg (target_os = "macos")] bsd_name : Option < Vec < u8 > > , pub (crate) file_system : OsString , pub (crate) mount_point : PathBuf , volume_url : CFRetained < CFURL > , pub (crate) total_space : u64 , pub (crate) available_space : u64 , pub (crate) is_removable : bool , pub (crate) is_read_only : bool , pub (crate) old_written_bytes : u64 , pub (crate) old_read_bytes : u64 , pub (crate) written_bytes : u64 , pub (crate) read_bytes : u64 , updated : bool , uuid : OsString , }
};
}
