// Generated macro for chown (function)
macro_rules! Depcrate_os_unix_fschown {
() => {
// Module: crate::os::unix::fs
// Provides: {"chown"}
// Dependencies: {}
# [doc = " Change the owner and group of the specified path."] # [doc = ""] # [doc = " Specifying either the uid or gid as `None` will leave it unchanged."] # [doc = ""] # [doc = " Changing the owner typically requires privileges, such as root or a specific capability."] # [doc = " Changing the group typically requires either being the owner and a member of the group, or"] # [doc = " having privileges."] # [doc = ""] # [doc = " Be aware that changing owner clears the `suid` and `sgid` permission bits in most cases"] # [doc = " according to POSIX, usually even if the user is root. The sgid is not cleared when"] # [doc = " the file is non-group-executable. See: <https://www.man7.org/linux/man-pages/man2/chown.2.html>"] # [doc = " This call may also clear file capabilities, if there was any."] # [doc = ""] # [doc = " If called on a symbolic link, this will change the owner and group of the link target. To"] # [doc = " change the owner and group of the link itself, see [`lchown`]."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```no_run"] # [doc = " use std::os::unix::fs;"] # [doc = ""] # [doc = " fn main() -> std::io::Result<()> {"] # [doc = "     fs::chown(\"/sandbox\", Some(0), Some(0))?;"] # [doc = "     Ok(())"] # [doc = " }"] # [doc = " ```"] # [stable (feature = "unix_chown" , since = "1.73.0")] pub fn chown < P : AsRef < Path > > (dir : P , uid : Option < u32 > , gid : Option < u32 >) -> io :: Result < () > { sys :: fs :: chown (dir . as_ref () , uid . unwrap_or (u32 :: MAX) , gid . unwrap_or (u32 :: MAX)) }
};
}
