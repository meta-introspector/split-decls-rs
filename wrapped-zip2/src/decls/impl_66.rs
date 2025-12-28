macro_rules! deps {
    () => {
        Ntfs!();
        ZipResult!();
        ZipError!();
    };
}

macro_rules! impl_66 {
    () => {
        deps!();
        impl Ntfs { # [doc = " Creates a NTFS extra field struct by reading the required bytes from the"] # [doc = " reader."] # [doc = ""] # [doc = " This method assumes that the length has already been read, therefore it"] # [doc = " must be passed as an argument."] pub fn try_from_reader < R > (reader : & mut R , len : u16) -> ZipResult < Self > where R : Read , { if len != 32 { return Err (ZipError :: UnsupportedArchive ("NTFS extra field has an unsupported length" ,)) ; } let _ = reader . read_u32_le () ? ; let tag = reader . read_u16_le () ? ; if tag != 0x0001 { return Err (ZipError :: UnsupportedArchive ("NTFS extra field has an unsupported attribute tag" ,)) ; } let size = reader . read_u16_le () ? ; if size != 24 { return Err (ZipError :: UnsupportedArchive ("NTFS extra field has an unsupported attribute size" ,)) ; } let mtime = reader . read_u64_le () ? ; let atime = reader . read_u64_le () ? ; let ctime = reader . read_u64_le () ? ; Ok (Self { mtime , atime , ctime , }) } # [doc = " Returns the file last modification time as a file time."] pub fn mtime (& self) -> u64 { self . mtime } # [doc = " Returns the file last modification time as a file time."] # [cfg (feature = "nt-time")] pub fn modified_file_time (& self) -> nt_time :: FileTime { nt_time :: FileTime :: new (self . mtime) } # [doc = " Returns the file last access time as a file time."] pub fn atime (& self) -> u64 { self . atime } # [doc = " Returns the file last access time as a file time."] # [cfg (feature = "nt-time")] pub fn accessed_file_time (& self) -> nt_time :: FileTime { nt_time :: FileTime :: new (self . atime) } # [doc = " Returns the file creation time as a file time."] pub fn ctime (& self) -> u64 { self . ctime } # [doc = " Returns the file creation time as a file time."] # [cfg (feature = "nt-time")] pub fn created_file_time (& self) -> nt_time :: FileTime { nt_time :: FileTime :: new (self . ctime) } }
    };
}

impl_66!()