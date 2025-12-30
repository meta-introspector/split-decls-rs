// Generated macro for stat (struct)
macro_rules! Depcrate_os_horizon_rawstat {
() => {
// Module: crate::os::horizon::raw
// Provides: {"stat"}
// Dependencies: {}
# [repr (C)] # [derive (Clone)] # [stable (feature = "raw_ext" , since = "1.1.0")] # [allow (dead_code)] pub struct stat { # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_dev : dev_t , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_ino : ino_t , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_mode : mode_t , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_nlink : nlink_t , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_uid : uid_t , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_gid : gid_t , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_rdev : dev_t , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_size : off_t , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_atime : time_t , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_mtime : time_t , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_ctime : time_t , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_blksize : blksize_t , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_blocks : blkcnt_t , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_spare4 : [c_long ; 2usize] , }
};
}
