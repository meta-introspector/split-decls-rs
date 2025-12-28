macro_rules! deps {
    () => {
        Magic!();
        ZipLocalEntryBlock!();
        ZipError!();
        FixedSizeBlock!();
    };
}

macro_rules! impl_214 {
    () => {
        deps!();
        impl FixedSizeBlock for ZipLocalEntryBlock { const MAGIC : spec :: Magic = spec :: Magic :: LOCAL_FILE_HEADER_SIGNATURE ; # [inline (always)] fn magic (self) -> spec :: Magic { self . magic } const WRONG_MAGIC_ERROR : ZipError = invalid ! ("Invalid local file header") ; to_and_from_le ! [(magic , spec :: Magic) , (version_made_by , u16) , (flags , u16) , (compression_method , u16) , (last_mod_time , u16) , (last_mod_date , u16) , (crc32 , u32) , (compressed_size , u32) , (uncompressed_size , u32) , (file_name_length , u16) , (extra_field_length , u16) ,] ; }
    };
}

impl_214!();