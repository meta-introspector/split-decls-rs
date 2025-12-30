// Generated macro for ZipError (enum)
macro_rules! Depcrate_resultZipError {
() => {
// Module: crate::result
// Provides: {"ZipError"}
// Dependencies: {}
# [doc = " Error type for Zip"] # [derive (Debug)] # [non_exhaustive] pub enum ZipError { # [doc = " i/o error"] Io (io :: Error) , # [doc = " invalid Zip archive"] InvalidArchive (Cow < 'static , str >) , # [doc = " unsupported Zip archive"] UnsupportedArchive (& 'static str) , # [doc = " specified file not found in archive"] FileNotFound , # [doc = " provided password is incorrect"] InvalidPassword , }
};
}
