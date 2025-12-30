// Generated macro for chroot (function)
macro_rules! Depcrate_os_unix_fschroot {
() => {
// Module: crate::os::unix::fs
// Provides: {"chroot"}
// Dependencies: {}
# [doc = " Change the root directory of the current process to the specified path."] # [doc = ""] # [doc = " This typically requires privileges, such as root or a specific capability."] # [doc = ""] # [doc = " This does not change the current working directory; you should call"] # [doc = " [`std::env::set_current_dir`][`crate::env::set_current_dir`] afterwards."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```no_run"] # [doc = " use std::os::unix::fs;"] # [doc = ""] # [doc = " fn main() -> std::io::Result<()> {"] # [doc = "     fs::chroot(\"/sandbox\")?;"] # [doc = "     std::env::set_current_dir(\"/\")?;"] # [doc = "     // continue working in sandbox"] # [doc = "     Ok(())"] # [doc = " }"] # [doc = " ```"] # [stable (feature = "unix_chroot" , since = "1.56.0")] # [cfg (not (target_os = "fuchsia"))] pub fn chroot < P : AsRef < Path > > (dir : P) -> io :: Result < () > { sys :: fs :: chroot (dir . as_ref ()) }
};
}
