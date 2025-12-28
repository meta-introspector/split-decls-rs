macro_rules! deps {
    () => {
        ZipCentralEntryBlock!();
        Magic!();
        ZipError!();
        FixedSizeBlock!();
    };
}

macro_rules! impl_211 {
    () => {
        deps!();
        impl FixedSizeBlock for ZipCentralEntryBlock { const MAGIC : spec :: Magic = spec :: Magic :: CENTRAL_DIRECTORY_HEADER_SIGNATURE ; # [inline (always)] fn magic (self) -> spec :: Magic { self . magic } const WRONG_MAGIC_ERROR : ZipError = invalid ! ("Invalid Central Directory header") ; to_and_from_le ! [(magic , spec :: Magic) , (version_made_by , u16) , (version_to_extract , u16) , (flags , u16) , (compression_method , u16) , (last_mod_time , u16) , (last_mod_date , u16) , (crc32 , u32) , (compressed_size , u32) , (uncompressed_size , u32) , (file_name_length , u16) , (extra_field_length , u16) , (file_comment_length , u16) , (disk_number , u16) , (internal_file_attributes , u16) , (external_file_attributes , u32) , (offset , u32) ,] ; }
    };
}

impl_211!();