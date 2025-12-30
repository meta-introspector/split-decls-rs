// Generated macro for symlink_metadata (function)
macro_rules! Depcrate_fssymlink_metadata {
() => {
// Module: crate::fs
// Provides: {"symlink_metadata"}
// Dependencies: {}
# [doc = " Queries the metadata about a file without following symlinks."] # [doc = ""] # [doc = " # Platform-specific behavior"] # [doc = ""] # [doc = " This function currently corresponds to the `lstat` function on Unix"] # [doc = " and the `GetFileInformationByHandle` function on Windows."] # [doc = " Note that, this [may change in the future][changes]."] # [doc = ""] # [doc = " [changes]: io#platform-specific-behavior"] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " This function will return an error in the following situations, but is not"] # [doc = " limited to just these cases:"] # [doc = ""] # [doc = " * The user lacks permissions to perform `metadata` call on `path`."] # [doc = " * `path` does not exist."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```rust,no_run"] # [doc = " use std::fs;"] # [doc = ""] # [doc = " fn main() -> std::io::Result<()> {"] # [doc = "     let attr = fs::symlink_metadata(\"/some/file/path.txt\")?;"] # [doc = "     // inspect attr ..."] # [doc = "     Ok(())"] # [doc = " }"] # [doc = " ```"] # [doc (alias = "lstat")] # [stable (feature = "symlink_metadata" , since = "1.1.0")] pub fn symlink_metadata < P : AsRef < Path > > (path : P) -> io :: Result < Metadata > { fs_imp :: symlink_metadata (path . as_ref ()) . map (Metadata) }
};
}
