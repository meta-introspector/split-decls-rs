macro_rules! Ntfs {
    () => {
        # [doc = " The NTFS extra field as described in [PKWARE's APPNOTE.TXT v6.3.9]."] # [doc = ""] # [doc = " This field stores [Windows file times], which are 64-bit unsigned integer"] # [doc = " values that represents the number of 100-nanosecond intervals that have"] # [doc = " elapsed since \"1601-01-01 00:00:00 UTC\"."] # [doc = ""] # [doc = " [PKWARE's APPNOTE.TXT v6.3.9]: https://pkware.cachefly.net/webdocs/casestudies/APPNOTE.TXT"] # [doc = " [Windows file times]: https://docs.microsoft.com/en-us/windows/win32/sysinfo/file-times"] # [derive (Clone , Debug)] pub struct Ntfs { mtime : u64 , atime : u64 , ctime : u64 , }
    };
}

Ntfs!();