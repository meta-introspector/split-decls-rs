// Generated macro for ZipFile (struct)
macro_rules! Depcrate_readZipFile {
() => {
// Module: crate::read
// Provides: {"ZipFile"}
// Dependencies: {}
# [doc = " A struct for reading a zip file"] pub struct ZipFile < 'a , R : Read > { pub (crate) data : Cow < 'a , ZipFileData > , pub (crate) reader : ZipFileReader < 'a , R > , }
};
}
