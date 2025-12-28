macro_rules! deps {
    () => {
        ZipFileData!();
        ZipCentralEntryBlock!();
        CentralDirectoryInfo!();
        ZipResult!();
    };
}

macro_rules! central_header_to_zip_file {
    () => {
        deps!();
        # [doc = " Parse a central directory entry to collect the information for the file."] pub (crate) fn central_header_to_zip_file < R : Read + Seek > (reader : & mut R , central_directory : & CentralDirectoryInfo ,) -> ZipResult < ZipFileData > { let central_header_start = reader . stream_position () ? ; let block = ZipCentralEntryBlock :: parse (reader) ? ; let file = central_header_to_zip_file_inner (reader , central_directory . archive_offset , central_header_start , block ,) ? ; let central_header_end = reader . stream_position () ? ; reader . seek (SeekFrom :: Start (central_header_end)) ? ; Ok (file) }
    };
}

central_header_to_zip_file!()