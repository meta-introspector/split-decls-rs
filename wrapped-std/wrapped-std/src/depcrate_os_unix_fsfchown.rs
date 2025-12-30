// Generated macro for fchown (function)
macro_rules! Depcrate_os_unix_fsfchown {
() => {
// Module: crate::os::unix::fs
// Provides: {"fchown"}
// Dependencies: {}
# [doc = " Change the owner and group of the file referenced by the specified open file descriptor."] # [doc = ""] # [doc = " For semantics and required privileges, see [`chown`]."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```no_run"] # [doc = " use std::os::unix::fs;"] # [doc = ""] # [doc = " fn main() -> std::io::Result<()> {"] # [doc = "     let f = std::fs::File::open(\"/file\")?;"] # [doc = "     fs::fchown(&f, Some(0), Some(0))?;"] # [doc = "     Ok(())"] # [doc = " }"] # [doc = " ```"] # [stable (feature = "unix_chown" , since = "1.73.0")] pub fn fchown < F : AsFd > (fd : F , uid : Option < u32 > , gid : Option < u32 >) -> io :: Result < () > { sys :: fs :: fchown (fd . as_fd () . as_raw_fd () , uid . unwrap_or (u32 :: MAX) , gid . unwrap_or (u32 :: MAX)) }
};
}
