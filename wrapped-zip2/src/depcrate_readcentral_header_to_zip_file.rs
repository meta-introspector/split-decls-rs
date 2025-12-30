// Generated macro for central_header_to_zip_file (function)
macro_rules! Depcrate_readcentral_header_to_zip_file {
() => {
// Module: crate::read
// Provides: {"central_header_to_zip_file"}
// Dependencies: {}
# [doc = " Parse a central directory entry to collect the information for the file."] pub (crate) fn central_header_to_zip_file < R : Read + Seek > (reader : & mut R , central_directory : & CentralDirectoryInfo ,) -> ZipResult < ZipFileData > { let central_header_start = reader . stream_position () ? ; let block = ZipCentralEntryBlock :: parse (reader) ? ; let file = central_header_to_zip_file_inner (reader , central_directory . archive_offset , central_header_start , block ,) ? ; let central_header_end = reader . stream_position () ? ; reader . seek (SeekFrom :: Start (central_header_end)) ? ; Ok (file) }
};
}
