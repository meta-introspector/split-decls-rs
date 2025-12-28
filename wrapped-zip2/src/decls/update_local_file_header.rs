macro_rules! deps {
    () => {
        ZipFileData!();
        ZipError!();
        ZipResult!();
    };
}

macro_rules! update_local_file_header {
    () => {
        deps!();
        fn update_local_file_header < T : Write + Seek > (writer : & mut T , file : & mut ZipFileData ,) -> ZipResult < () > { const CRC32_OFFSET : u64 = 14 ; writer . seek (SeekFrom :: Start (file . header_start + CRC32_OFFSET)) ? ; writer . write_u32_le (file . crc32) ? ; if file . large_file { writer . write_u32_le (spec :: ZIP64_BYTES_THR as u32) ? ; writer . write_u32_le (spec :: ZIP64_BYTES_THR as u32) ? ; update_local_zip64_extra_field (writer , file) ? ; file . compressed_size = spec :: ZIP64_BYTES_THR ; file . uncompressed_size = spec :: ZIP64_BYTES_THR ; } else { if file . compressed_size > spec :: ZIP64_BYTES_THR { return Err (ZipError :: Io (io :: Error :: other ("Large file option has not been set" ,))) ; } writer . write_u32_le (file . compressed_size as u32) ? ; writer . write_u32_le (file . uncompressed_size as u32) ? ; } Ok (()) }
    };
}

update_local_file_header!()