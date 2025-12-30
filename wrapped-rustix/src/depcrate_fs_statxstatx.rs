// Generated macro for Statx (struct)
macro_rules! Depcrate_fs_statxStatx {
() => {
// Module: crate::fs::statx
// Provides: {"Statx"}
// Dependencies: {}
# [doc = " `struct statx` for use with [`statx`]."] # [repr (C)] # [derive (Debug , Copy , Clone)] # [allow (missing_docs)] # [non_exhaustive] pub struct Statx { pub stx_mask : u32 , pub stx_blksize : u32 , pub stx_attributes : StatxAttributes , pub stx_nlink : u32 , pub stx_uid : u32 , pub stx_gid : u32 , pub stx_mode : u16 , pub (crate) __spare0 : [u16 ; 1] , pub stx_ino : u64 , pub stx_size : u64 , pub stx_blocks : u64 , pub stx_attributes_mask : StatxAttributes , pub stx_atime : StatxTimestamp , pub stx_btime : StatxTimestamp , pub stx_ctime : StatxTimestamp , pub stx_mtime : StatxTimestamp , pub stx_rdev_major : u32 , pub stx_rdev_minor : u32 , pub stx_dev_major : u32 , pub stx_dev_minor : u32 , pub stx_mnt_id : u64 , pub stx_dio_mem_align : u32 , pub stx_dio_offset_align : u32 , pub stx_subvol : u64 , pub stx_atomic_write_unit_min : u32 , pub stx_atomic_write_unit_max : u32 , pub stx_atomic_write_segments_max : u32 , pub stx_dio_read_offset_align : u32 , pub stx_atomic_write_unit_max_opt : u32 , pub __spare2 : [u32 ; 1usize] , pub __spare3 : [u64 ; 8usize] , }
};
}
