macro_rules! deps {
    () => {
        ZipFileData!();
        ZipError!();
        CompressionMethod!();
        ZipResult!();
        System!();
        DateTime!();
        ZipCentralEntryBlock!();
    };
}

macro_rules! central_header_to_zip_file_inner {
    () => {
        deps!();
        # [doc = " Parse a central directory entry to collect the information for the file."] fn central_header_to_zip_file_inner < R : Read > (reader : & mut R , archive_offset : u64 , central_header_start : u64 , block : ZipCentralEntryBlock ,) -> ZipResult < ZipFileData > { let ZipCentralEntryBlock { version_made_by , flags , compression_method , last_mod_time , last_mod_date , crc32 , compressed_size , uncompressed_size , file_name_length , extra_field_length , file_comment_length , external_file_attributes , offset , .. } = block ; let encrypted = flags & 1 == 1 ; let is_utf8 = flags & (1 << 11) != 0 ; let using_data_descriptor = flags & (1 << 3) != 0 ; let file_name_raw = read_variable_length_byte_field (reader , file_name_length as usize) ? ; let extra_field = read_variable_length_byte_field (reader , extra_field_length as usize) ? ; let file_comment_raw = read_variable_length_byte_field (reader , file_comment_length as usize) ? ; let file_name : Box < str > = match is_utf8 { true => String :: from_utf8_lossy (& file_name_raw) . into () , false => file_name_raw . clone () . from_cp437 () , } ; let file_comment : Box < str > = match is_utf8 { true => String :: from_utf8_lossy (& file_comment_raw) . into () , false => file_comment_raw . from_cp437 () , } ; let mut result = ZipFileData { system : System :: from ((version_made_by >> 8) as u8) , version_made_by : version_made_by as u8 , encrypted , using_data_descriptor , is_utf8 , compression_method : CompressionMethod :: parse_from_u16 (compression_method) , compression_level : None , last_modified_time : DateTime :: try_from_msdos (last_mod_date , last_mod_time) . ok () , crc32 , compressed_size : compressed_size . into () , uncompressed_size : uncompressed_size . into () , flags , file_name , file_name_raw , extra_field : Some (Arc :: new (extra_field . to_vec ())) , central_extra_field : None , file_comment , header_start : offset . into () , extra_data_start : None , central_header_start , data_start : OnceLock :: new () , external_attributes : external_file_attributes , large_file : false , aes_mode : None , aes_extra_data_start : 0 , extra_fields : Vec :: new () , } ; match parse_extra_field (& mut result) { Ok (stripped_extra_field) => { result . extra_field = stripped_extra_field ; } Err (ZipError :: Io (..)) => { } Err (e) => return Err (e) , } let aes_enabled = result . compression_method == CompressionMethod :: AES ; if aes_enabled && result . aes_mode . is_none () { return Err (invalid ! ("AES encryption without AES extra data field")) ; } result . header_start = result . header_start . checked_add (archive_offset) . ok_or (invalid ! ("Archive header is too large")) ? ; Ok (result) }
    };
}

central_header_to_zip_file_inner!();