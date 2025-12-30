// Generated macro for ioctl_fionread (function)
macro_rules! Depcrate_io_ioctlioctl_fionread {
() => {
// Module: crate::io::ioctl
// Provides: {"ioctl_fionread"}
// Dependencies: {}
# [doc = " `ioctl(fd, FIONREAD)`—Returns the number of bytes ready to be read."] # [doc = ""] # [doc = " The result of this function gets silently coerced into a C `int` by the OS,"] # [doc = " so it may contain a wrapped value."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux]"] # [doc = "  - [Winsock]"] # [doc = "  - [FreeBSD]"] # [doc = "  - [NetBSD]"] # [doc = "  - [OpenBSD]"] # [doc = ""] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/ioctl_tty.2.html"] # [doc = " [Winsock]: https://docs.microsoft.com/en-us/windows/win32/winsock/winsock-ioctls#unix-ioctl-codes"] # [doc = " [FreeBSD]: https://man.freebsd.org/cgi/man.cgi?query=ioctl&sektion=2#GENERIC%09IOCTLS"] # [doc = " [NetBSD]: https://man.netbsd.org/ioctl.2#GENERIC%20IOCTLS"] # [doc = " [OpenBSD]: https://man.openbsd.org/ioctl.2#GENERIC_IOCTLS"] # [cfg (not (any (target_os = "espidf" , target_os = "horizon" , target_os = "vita")))] # [inline] # [doc (alias = "FIONREAD")] pub fn ioctl_fionread < Fd : AsFd > (fd : Fd) -> io :: Result < u64 > { unsafe { let ctl = ioctl :: Getter :: < { c :: FIONREAD } , c :: c_int > :: new () ; ioctl :: ioctl (fd , ctl) . map (| n | n as u64) } }
};
}
