// Generated macro for soft_link (function)
macro_rules! Depcrate_fssoft_link {
() => {
// Module: crate::fs
// Provides: {"soft_link"}
// Dependencies: {}
# [doc = " Creates a new symbolic link on the filesystem."] # [doc = ""] # [doc = " The `link` path will be a symbolic link pointing to the `original` path."] # [doc = " On Windows, this will be a file symlink, not a directory symlink;"] # [doc = " for this reason, the platform-specific [`std::os::unix::fs::symlink`]"] # [doc = " and [`std::os::windows::fs::symlink_file`] or [`symlink_dir`] should be"] # [doc = " used instead to make the intent explicit."] # [doc = ""] # [doc = " [`std::os::unix::fs::symlink`]: crate::os::unix::fs::symlink"] # [doc = " [`std::os::windows::fs::symlink_file`]: crate::os::windows::fs::symlink_file"] # [doc = " [`symlink_dir`]: crate::os::windows::fs::symlink_dir"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```no_run"] # [doc = " use std::fs;"] # [doc = ""] # [doc = " fn main() -> std::io::Result<()> {"] # [doc = "     fs::soft_link(\"a.txt\", \"b.txt\")?;"] # [doc = "     Ok(())"] # [doc = " }"] # [doc = " ```"] # [stable (feature = "rust1" , since = "1.0.0")] # [deprecated (since = "1.1.0" , note = "replaced with std::os::unix::fs::symlink and \
            std::os::windows::fs::{symlink_file, symlink_dir}")] pub fn soft_link < P : AsRef < Path > , Q : AsRef < Path > > (original : P , link : Q) -> io :: Result < () > { fs_imp :: symlink (original . as_ref () , link . as_ref ()) }
};
}
