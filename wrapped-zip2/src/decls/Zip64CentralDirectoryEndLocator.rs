macro_rules! Zip64CentralDirectoryEndLocator {
    () => {
        pub (crate) struct Zip64CentralDirectoryEndLocator { pub disk_with_central_directory : u32 , pub end_of_central_directory_offset : u64 , pub number_of_disks : u32 , }
    };
}

Zip64CentralDirectoryEndLocator!();