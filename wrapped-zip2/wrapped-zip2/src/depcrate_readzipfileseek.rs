// Generated macro for ZipFileSeek (struct)
macro_rules! Depcrate_readZipFileSeek {
() => {
// Module: crate::read
// Provides: {"ZipFileSeek"}
// Dependencies: {}
# [doc = " A struct for reading and seeking a zip file"] pub struct ZipFileSeek < 'a , R > { data : Cow < 'a , ZipFileData > , reader : ZipFileSeekReader < 'a , R > , }
};
}
