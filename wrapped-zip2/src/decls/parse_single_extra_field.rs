macro_rules! deps {
    () => {
        Aes128!();
        Aes192!();
        Aes256!();
        AesVendorVersion!();
        UnicodeExtraField!();
        ExtraField!();
        ZipError!();
        ExtendedTimestamp!();
        AesMode!();
        CompressionMethod!();
        Ntfs!();
        ZipFileData!();
        ZipResult!();
    };
}

macro_rules! parse_single_extra_field {
    () => {
        deps!();
        pub (crate) fn parse_single_extra_field < R : Read > (file : & mut ZipFileData , reader : & mut R , bytes_already_read : u64 , disallow_zip64 : bool ,) -> ZipResult < bool > { let kind = reader . read_u16_le () ? ; let len = reader . read_u16_le () ? ; match kind { 0x0001 => { if disallow_zip64 { return Err (invalid ! ("Can't write a custom field using the ZIP64 ID")) ; } file . large_file = true ; let mut consumed_len = 0 ; if len >= 24 || file . uncompressed_size == spec :: ZIP64_BYTES_THR { file . uncompressed_size = reader . read_u64_le () ? ; consumed_len += size_of :: < u64 > () ; } if len >= 24 || file . compressed_size == spec :: ZIP64_BYTES_THR { file . compressed_size = reader . read_u64_le () ? ; consumed_len += size_of :: < u64 > () ; } if len >= 24 || file . header_start == spec :: ZIP64_BYTES_THR { file . header_start = reader . read_u64_le () ? ; consumed_len += size_of :: < u64 > () ; } let Some (leftover_len) = (len as usize) . checked_sub (consumed_len) else { return Err (invalid ! ("ZIP64 extra-data field is the wrong length")) ; } ; reader . read_exact (& mut vec ! [0u8 ; leftover_len]) ? ; return Ok (true) ; } 0x000a => { file . extra_fields . push (ExtraField :: Ntfs (Ntfs :: try_from_reader (reader , len) ?)) ; } 0x9901 => { if len != 7 { return Err (ZipError :: UnsupportedArchive ("AES extra data field has an unsupported length" ,)) ; } let vendor_version = reader . read_u16_le () ? ; let vendor_id = reader . read_u16_le () ? ; let mut out = [0u8] ; reader . read_exact (& mut out) ? ; let aes_mode = out [0] ; let compression_method = CompressionMethod :: parse_from_u16 (reader . read_u16_le () ?) ; if vendor_id != 0x4541 { return Err (invalid ! ("Invalid AES vendor")) ; } let vendor_version = match vendor_version { 0x0001 => AesVendorVersion :: Ae1 , 0x0002 => AesVendorVersion :: Ae2 , _ => return Err (invalid ! ("Invalid AES vendor version")) , } ; match aes_mode { 0x01 => file . aes_mode = Some ((AesMode :: Aes128 , vendor_version , compression_method)) , 0x02 => file . aes_mode = Some ((AesMode :: Aes192 , vendor_version , compression_method)) , 0x03 => file . aes_mode = Some ((AesMode :: Aes256 , vendor_version , compression_method)) , _ => return Err (invalid ! ("Invalid AES encryption strength")) , } ; file . compression_method = compression_method ; file . aes_extra_data_start = bytes_already_read ; } 0x5455 => { file . extra_fields . push (ExtraField :: ExtendedTimestamp (ExtendedTimestamp :: try_from_reader (reader , len) ? ,)) ; } 0x6375 => { file . file_comment = String :: from_utf8 (UnicodeExtraField :: try_from_reader (reader , len) ? . unwrap_valid (file . file_comment . as_bytes ()) ? . into_vec () ,) ? . into () ; } 0x7075 => { file . file_name_raw = UnicodeExtraField :: try_from_reader (reader , len) ? . unwrap_valid (& file . file_name_raw) ? ; file . file_name = String :: from_utf8 (file . file_name_raw . clone () . into_vec ()) ? . into_boxed_str () ; file . is_utf8 = true ; } _ => { reader . read_exact (& mut vec ! [0u8 ; len as usize]) ? ; } } Ok (false) }
    };
}

parse_single_extra_field!();