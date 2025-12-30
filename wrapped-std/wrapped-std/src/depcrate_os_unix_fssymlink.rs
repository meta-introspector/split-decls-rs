// Generated macro for symlink (function)
macro_rules! Depcrate_os_unix_fssymlink {
() => {
// Module: crate::os::unix::fs
// Provides: {"symlink"}
// Dependencies: {}
# [doc = " Creates a new symbolic link on the filesystem."] # [doc = ""] # [doc = " The `link` path will be a symbolic link pointing to the `original` path."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```no_run"] # [doc = " use std::os::unix::fs;"] # [doc = ""] # [doc = " fn main() -> std::io::Result<()> {"] # [doc = "     fs::symlink(\"a.txt\", \"b.txt\")?;"] # [doc = "     Ok(())"] # [doc = " }"] # [doc = " ```"] # [stable (feature = "symlink" , since = "1.1.0")] pub fn symlink < P : AsRef < Path > , Q : AsRef < Path > > (original : P , link : Q) -> io :: Result < () > { sys :: fs :: symlink (original . as_ref () , link . as_ref ()) }
};
}
