// Generated macro for ioctl_fioclex (function)
macro_rules! Depcrate_io_ioctlioctl_fioclex {
() => {
// Module: crate::io::ioctl
// Provides: {"ioctl_fioclex"}
// Dependencies: {}
# [doc = " `ioctl(fd, FIOCLEX, NULL)`—Set the close-on-exec flag."] # [doc = ""] # [doc = " This is similar to `fcntl(fd, F_SETFD, FD_CLOEXEC)`, except that it avoids"] # [doc = " clearing any other flags that might be set."] # [doc = ""] # [doc = " Linux: Note that `ioctl` can not be used on `OFlags::PATH` file"] # [doc = " descriptors."] # [cfg (any (apple , linux_kernel))] # [inline] # [doc (alias = "FIOCLEX")] # [doc (alias = "FD_CLOEXEC")] pub fn ioctl_fioclex < Fd : AsFd > (fd : Fd) -> io :: Result < () > { unsafe { let ctl = ioctl :: NoArg :: < { c :: FIOCLEX } > :: new () ; ioctl :: ioctl (fd , ctl) } }
};
}
