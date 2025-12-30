// Generated macro for stat (struct)
macro_rules! Depcrate_os_emscripten_rawstat {
() => {
// Module: crate::os::emscripten::raw
// Provides: {"stat"}
// Dependencies: {}
# [repr (C)] # [derive (Clone)] # [stable (feature = "raw_ext" , since = "1.1.0")] pub struct stat { # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_dev : u64 , # [stable (feature = "raw_ext" , since = "1.1.0")] pub __pad1 : c_short , # [stable (feature = "raw_ext" , since = "1.1.0")] pub __st_ino : u32 , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_mode : u32 , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_nlink : u32 , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_uid : u32 , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_gid : u32 , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_rdev : u64 , # [stable (feature = "raw_ext" , since = "1.1.0")] pub __pad2 : c_uint , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_size : i64 , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_blksize : i32 , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_blocks : i64 , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_atime : time_t , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_atime_nsec : c_long , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_mtime : time_t , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_mtime_nsec : c_long , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_ctime : time_t , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_ctime_nsec : c_long , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_ino : u64 , }
};
}
