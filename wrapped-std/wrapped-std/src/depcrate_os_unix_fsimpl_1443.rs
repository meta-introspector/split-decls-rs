// Generated macro for impl_1443 (impl)
macro_rules! Depcrate_os_unix_fsimpl_1443 {
() => {
// Module: crate::os::unix::fs
// Provides: {"impl_1443"}
// Dependencies: {}
# [stable (feature = "metadata_ext" , since = "1.1.0")] impl MetadataExt for fs :: Metadata { fn dev (& self) -> u64 { self . st_dev () } fn ino (& self) -> u64 { self . st_ino () } fn mode (& self) -> u32 { self . st_mode () } fn nlink (& self) -> u64 { self . st_nlink () } fn uid (& self) -> u32 { self . st_uid () } fn gid (& self) -> u32 { self . st_gid () } fn rdev (& self) -> u64 { self . st_rdev () } fn size (& self) -> u64 { self . st_size () } fn atime (& self) -> i64 { self . st_atime () } fn atime_nsec (& self) -> i64 { self . st_atime_nsec () } fn mtime (& self) -> i64 { self . st_mtime () } fn mtime_nsec (& self) -> i64 { self . st_mtime_nsec () } fn ctime (& self) -> i64 { self . st_ctime () } fn ctime_nsec (& self) -> i64 { self . st_ctime_nsec () } fn blksize (& self) -> u64 { self . st_blksize () } fn blocks (& self) -> u64 { self . st_blocks () } # [cfg (target_os = "vxworks")] fn attrib (& self) -> u8 { self . st_attrib () } }
};
}
