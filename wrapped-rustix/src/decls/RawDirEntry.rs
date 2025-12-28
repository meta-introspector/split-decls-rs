macro_rules! RawDirEntry {
    () => {
        # [doc = " A raw directory entry, similar to [`std::fs::DirEntry`]."] # [doc = ""] # [doc = " Unlike the std version, this may represent the `.` or `..` entries."] pub struct RawDirEntry < 'a > { file_name : & 'a CStr , file_type : u8 , inode_number : u64 , next_entry_cookie : i64 , }
    };
}

RawDirEntry!();