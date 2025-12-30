// Generated macro for HasZipMetadata (trait)
macro_rules! Depcrate_readHasZipMetadata {
() => {
// Module: crate::read
// Provides: {"HasZipMetadata"}
// Dependencies: {}
# [doc = " A trait for exposing file metadata inside the zip."] pub trait HasZipMetadata { # [doc = " Get the file metadata"] fn get_metadata (& self) -> & ZipFileData ; }
};
}
