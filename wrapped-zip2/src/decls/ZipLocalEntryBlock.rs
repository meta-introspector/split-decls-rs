macro_rules! deps {
    () => {
        Magic!();
    };
}

macro_rules! ZipLocalEntryBlock {
    () => {
        deps!();
        # [derive (Copy , Clone , Debug)] # [repr (packed , C)] pub (crate) struct ZipLocalEntryBlock { magic : spec :: Magic , pub version_made_by : u16 , pub flags : u16 , pub compression_method : u16 , pub last_mod_time : u16 , pub last_mod_date : u16 , pub crc32 : u32 , pub compressed_size : u32 , pub uncompressed_size : u32 , pub file_name_length : u16 , pub extra_field_length : u16 , }
    };
}

ZipLocalEntryBlock!();