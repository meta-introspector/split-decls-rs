// Generated macro for metadata (function)
macro_rules! Depcrate_fsmetadata {
() => {
// Module: crate::fs
// Provides: {"metadata"}
// Dependencies: {}
# [doc = " Given a path, queries the file system to get information about a file,"] # [doc = " directory, etc."] # [doc = ""] # [doc = " This function will traverse symbolic links to query information about the"] # [doc = " destination file."] # [doc = ""] # [doc = " # Platform-specific behavior"] # [doc = ""] # [doc = " This function currently corresponds to the `stat` function on Unix"] # [doc = " and the `GetFileInformationByHandle` function on Windows."] # [doc = " Note that, this [may change in the future][changes]."] # [doc = ""] # [doc = " [changes]: io#platform-specific-behavior"] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " This function will return an error in the following situations, but is not"] # [doc = " limited to just these cases:"] # [doc = ""] # [doc = " * The user lacks permissions to perform `metadata` call on `path`."] # [doc = " * `path` does not exist."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```rust,no_run"] # [doc = " use std::fs;"] # [doc = ""] # [doc = " fn main() -> std::io::Result<()> {"] # [doc = "     let attr = fs::metadata(\"/some/file/path.txt\")?;"] # [doc = "     // inspect attr ..."] # [doc = "     Ok(())"] # [doc = " }"] # [doc = " ```"] # [doc (alias = "stat")] # [stable (feature = "rust1" , since = "1.0.0")] pub fn metadata < P : AsRef < Path > > (path : P) -> io :: Result < Metadata > { fs_imp :: metadata (path . as_ref ()) . map (Metadata) }
};
}
