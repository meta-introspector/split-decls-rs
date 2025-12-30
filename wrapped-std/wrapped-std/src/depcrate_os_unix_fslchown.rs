// Generated macro for lchown (function)
macro_rules! Depcrate_os_unix_fslchown {
() => {
// Module: crate::os::unix::fs
// Provides: {"lchown"}
// Dependencies: {}
# [doc = " Change the owner and group of the specified path, without dereferencing symbolic links."] # [doc = ""] # [doc = " Identical to [`chown`], except that if called on a symbolic link, this will change the owner"] # [doc = " and group of the link itself rather than the owner and group of the link target."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```no_run"] # [doc = " use std::os::unix::fs;"] # [doc = ""] # [doc = " fn main() -> std::io::Result<()> {"] # [doc = "     fs::lchown(\"/symlink\", Some(0), Some(0))?;"] # [doc = "     Ok(())"] # [doc = " }"] # [doc = " ```"] # [stable (feature = "unix_chown" , since = "1.73.0")] pub fn lchown < P : AsRef < Path > > (dir : P , uid : Option < u32 > , gid : Option < u32 >) -> io :: Result < () > { sys :: fs :: lchown (dir . as_ref () , uid . unwrap_or (u32 :: MAX) , gid . unwrap_or (u32 :: MAX)) }
};
}
