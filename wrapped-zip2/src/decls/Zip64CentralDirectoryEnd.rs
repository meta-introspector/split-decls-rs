macro_rules! Zip64CentralDirectoryEnd {
    () => {
        pub (crate) struct Zip64CentralDirectoryEnd { pub record_size : u64 , pub version_made_by : u16 , pub version_needed_to_extract : u16 , pub disk_number : u32 , pub disk_with_central_directory : u32 , pub number_of_files_on_this_disk : u64 , pub number_of_files : u64 , pub central_directory_size : u64 , pub central_directory_offset : u64 , pub extensible_data_sector : Box < [u8] > , }
    };
}

Zip64CentralDirectoryEnd!()