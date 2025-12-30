// Generated macro for stat (struct)
macro_rules! Depcrate_os_openbsd_rawstat {
() => {
// Module: crate::os::openbsd::raw
// Provides: {"stat"}
// Dependencies: {}
# [repr (C)] # [derive (Clone)] # [stable (feature = "raw_ext" , since = "1.1.0")] pub struct stat { # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_mode : u32 , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_dev : i32 , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_ino : u64 , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_nlink : u32 , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_uid : u32 , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_gid : u32 , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_rdev : i32 , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_atime : i64 , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_atime_nsec : c_long , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_mtime : i64 , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_mtime_nsec : c_long , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_ctime : i64 , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_ctime_nsec : c_long , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_size : i64 , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_blocks : i64 , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_blksize : i32 , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_flags : u32 , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_gen : u32 , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_birthtime : i64 , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_birthtime_nsec : c_long , }
};
}
