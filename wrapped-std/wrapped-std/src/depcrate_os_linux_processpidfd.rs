// Generated macro for PidFd (struct)
macro_rules! Depcrate_os_linux_processPidFd {
() => {
// Module: crate::os::linux::process
// Provides: {"PidFd"}
// Dependencies: {}
# [doc = " This type represents a file descriptor that refers to a process."] # [doc = ""] # [doc = " A `PidFd` can be obtained by setting the corresponding option on [`Command`]"] # [doc = " with [`create_pidfd`]. Subsequently, the created pidfd can be retrieved"] # [doc = " from the [`Child`] by calling [`pidfd`] or [`into_pidfd`]."] # [doc = ""] # [doc = " Example:"] # [doc = " ```no_run"] # [doc = " #![feature(linux_pidfd)]"] # [doc = " use std::os::linux::process::{CommandExt, ChildExt};"] # [doc = " use std::process::Command;"] # [doc = ""] # [doc = " let mut child = Command::new(\"echo\")"] # [doc = "     .create_pidfd(true)"] # [doc = "     .spawn()"] # [doc = "     .expect(\"Failed to spawn child\");"] # [doc = ""] # [doc = " let pidfd = child"] # [doc = "     .into_pidfd()"] # [doc = "     .expect(\"Failed to retrieve pidfd\");"] # [doc = ""] # [doc = " // The file descriptor will be closed when `pidfd` is dropped."] # [doc = " ```"] # [doc = " Refer to the man page of [`pidfd_open(2)`] for further details."] # [doc = ""] # [doc = " [`Command`]: process::Command"] # [doc = " [`create_pidfd`]: CommandExt::create_pidfd"] # [doc = " [`Child`]: process::Child"] # [doc = " [`pidfd`]: fn@ChildExt::pidfd"] # [doc = " [`into_pidfd`]: ChildExt::into_pidfd"] # [doc = " [`pidfd_open(2)`]: https://man7.org/linux/man-pages/man2/pidfd_open.2.html"] # [derive (Debug)] # [repr (transparent)] pub struct PidFd { inner : InnerPidFd , }
};
}
