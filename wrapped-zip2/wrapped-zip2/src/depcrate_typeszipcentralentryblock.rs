// Generated macro for ZipCentralEntryBlock (struct)
macro_rules! Depcrate_typesZipCentralEntryBlock {
() => {
// Module: crate::types
// Provides: {"ZipCentralEntryBlock"}
// Dependencies: {}
# [derive (Copy , Clone , Debug)] # [repr (packed , C)] pub (crate) struct ZipCentralEntryBlock { magic : spec :: Magic , pub version_made_by : u16 , pub version_to_extract : u16 , pub flags : u16 , pub compression_method : u16 , pub last_mod_time : u16 , pub last_mod_date : u16 , pub crc32 : u32 , pub compressed_size : u32 , pub uncompressed_size : u32 , pub file_name_length : u16 , pub extra_field_length : u16 , pub file_comment_length : u16 , pub disk_number : u16 , pub internal_file_attributes : u16 , pub external_file_attributes : u32 , pub offset : u32 , }
};
}
