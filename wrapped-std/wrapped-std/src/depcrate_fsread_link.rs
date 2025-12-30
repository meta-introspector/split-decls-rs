// Generated macro for read_link (function)
macro_rules! Depcrate_fsread_link {
() => {
// Module: crate::fs
// Provides: {"read_link"}
// Dependencies: {}
# [doc = " Reads a symbolic link, returning the file that the link points to."] # [doc = ""] # [doc = " # Platform-specific behavior"] # [doc = ""] # [doc = " This function currently corresponds to the `readlink` function on Unix"] # [doc = " and the `CreateFile` function with `FILE_FLAG_OPEN_REPARSE_POINT` and"] # [doc = " `FILE_FLAG_BACKUP_SEMANTICS` flags on Windows."] # [doc = " Note that, this [may change in the future][changes]."] # [doc = ""] # [doc = " [changes]: io#platform-specific-behavior"] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " This function will return an error in the following situations, but is not"] # [doc = " limited to just these cases:"] # [doc = ""] # [doc = " * `path` is not a symbolic link."] # [doc = " * `path` does not exist."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```no_run"] # [doc = " use std::fs;"] # [doc = ""] # [doc = " fn main() -> std::io::Result<()> {"] # [doc = "     let path = fs::read_link(\"a.txt\")?;"] # [doc = "     Ok(())"] # [doc = " }"] # [doc = " ```"] # [stable (feature = "rust1" , since = "1.0.0")] pub fn read_link < P : AsRef < Path > > (path : P) -> io :: Result < PathBuf > { fs_imp :: read_link (path . as_ref ()) }
};
}
