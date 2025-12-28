macro_rules! CentralDirectoryInfo {
    () => {
        # [derive (Debug)] pub (crate) struct CentralDirectoryInfo { pub (crate) archive_offset : u64 , pub (crate) directory_start : u64 , pub (crate) number_of_files : usize , pub (crate) disk_number : u32 , pub (crate) disk_with_central_directory : u32 , }
    };
}

CentralDirectoryInfo!()