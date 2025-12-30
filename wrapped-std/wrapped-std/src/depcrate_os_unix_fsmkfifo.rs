// Generated macro for mkfifo (function)
macro_rules! Depcrate_os_unix_fsmkfifo {
() => {
// Module: crate::os::unix::fs
// Provides: {"mkfifo"}
// Dependencies: {}
# [doc = " Create a FIFO special file at the specified path with the specified mode."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```no_run"] # [doc = " # #![feature(unix_mkfifo)]"] # [doc = " # #[cfg(not(unix))]"] # [doc = " # fn main() {}"] # [doc = " # #[cfg(unix)]"] # [doc = " # fn main() -> std::io::Result<()> {"] # [doc = " # use std::{"] # [doc = " #     os::unix::fs::{mkfifo, PermissionsExt},"] # [doc = " #     fs::{File, Permissions, remove_file},"] # [doc = " #     io::{Write, Read},"] # [doc = " # };"] # [doc = " # let _ = remove_file(\"/tmp/fifo\");"] # [doc = " mkfifo(\"/tmp/fifo\", Permissions::from_mode(0o774))?;"] # [doc = ""] # [doc = " let mut wx = File::options().read(true).write(true).open(\"/tmp/fifo\")?;"] # [doc = " let mut rx = File::open(\"/tmp/fifo\")?;"] # [doc = ""] # [doc = " wx.write_all(b\"hello, world!\")?;"] # [doc = " drop(wx);"] # [doc = ""] # [doc = " let mut s = String::new();"] # [doc = " rx.read_to_string(&mut s)?;"] # [doc = ""] # [doc = " assert_eq!(s, \"hello, world!\");"] # [doc = " # Ok(())"] # [doc = " # }"] # [doc = " ```"] # [unstable (feature = "unix_mkfifo" , issue = "139324")] pub fn mkfifo < P : AsRef < Path > > (path : P , permissions : Permissions) -> io :: Result < () > { sys :: fs :: mkfifo (path . as_ref () , permissions . mode ()) }
};
}
