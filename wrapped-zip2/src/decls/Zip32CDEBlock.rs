macro_rules! deps {
    () => {
        Magic!();
    };
}

macro_rules! Zip32CDEBlock {
    () => {
        deps!();
        # [derive (Copy , Clone , Debug)] # [repr (packed , C)] pub (crate) struct Zip32CDEBlock { magic : Magic , pub disk_number : u16 , pub disk_with_central_directory : u16 , pub number_of_files_on_this_disk : u16 , pub number_of_files : u16 , pub central_directory_size : u32 , pub central_directory_offset : u32 , pub zip_file_comment_length : u16 , }
    };
}

Zip32CDEBlock!();