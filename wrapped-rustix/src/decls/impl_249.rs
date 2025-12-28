macro_rules! deps {
    () => {
        RawDirEntry!();
    };
}

macro_rules! impl_249 {
    () => {
        deps!();
        impl < 'a > RawDirEntry < 'a > { # [doc = " Returns the file name of this directory entry."] # [inline] pub fn file_name (& self) -> & CStr { self . file_name } # [doc = " Returns the type of this directory entry."] # [inline] pub fn file_type (& self) -> FileType { FileType :: from_dirent_d_type (self . file_type) } # [doc = " Returns the inode number of this directory entry."] # [inline] # [doc (alias = "inode_number")] pub fn ino (& self) -> u64 { self . inode_number } # [doc = " Returns the seek cookie to the next directory entry."] # [inline] # [doc (alias = "off")] pub fn next_entry_cookie (& self) -> u64 { self . next_entry_cookie as u64 } }
    };
}

impl_249!()