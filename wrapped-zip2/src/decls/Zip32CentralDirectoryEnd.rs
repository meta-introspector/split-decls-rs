macro_rules! Zip32CentralDirectoryEnd {
    () => {
        # [derive (Debug)] pub (crate) struct Zip32CentralDirectoryEnd { pub disk_number : u16 , pub disk_with_central_directory : u16 , pub number_of_files_on_this_disk : u16 , pub number_of_files : u16 , pub central_directory_size : u32 , pub central_directory_offset : u32 , pub zip_file_comment : Box < [u8] > , }
    };
}

Zip32CentralDirectoryEnd!()