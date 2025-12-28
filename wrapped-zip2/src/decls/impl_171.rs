macro_rules! deps {
    () => {
        Magic!();
        FixedSizeBlock!();
        Zip64CDEBlock!();
        ZipError!();
    };
}

macro_rules! impl_171 {
    () => {
        deps!();
        impl FixedSizeBlock for Zip64CDEBlock { const MAGIC : Magic = Magic :: ZIP64_CENTRAL_DIRECTORY_END_SIGNATURE ; fn magic (self) -> Magic { self . magic } const WRONG_MAGIC_ERROR : ZipError = invalid ! ("Invalid digital signature header") ; to_and_from_le ! [(magic , Magic) , (record_size , u64) , (version_made_by , u16) , (version_needed_to_extract , u16) , (disk_number , u32) , (disk_with_central_directory , u32) , (number_of_files_on_this_disk , u64) , (number_of_files , u64) , (central_directory_size , u64) , (central_directory_offset , u64) ,] ; }
    };
}

impl_171!();