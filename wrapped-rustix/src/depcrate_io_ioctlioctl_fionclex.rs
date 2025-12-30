// Generated macro for ioctl_fionclex (function)
macro_rules! Depcrate_io_ioctlioctl_fionclex {
() => {
// Module: crate::io::ioctl
// Provides: {"ioctl_fionclex"}
// Dependencies: {}
# [doc = " `ioctl(fd, FIONCLEX, NULL)`—Remove the close-on-exec flag."] # [doc = ""] # [doc = " This is similar to `fcntl_setfd(fd, FdFlags::empty())`, except that it avoids"] # [doc = " clearing any other flags that might be set."] # [doc = ""] # [doc = " Linux: Note that `ioctl` can not be used on `OFlags::PATH` file"] # [doc = " descriptors."] # [cfg (any (apple , linux_kernel))] # [inline] # [doc (alias = "FIONCLEX")] pub fn ioctl_fionclex < Fd : AsFd > (fd : Fd) -> io :: Result < () > { unsafe { let ctl = ioctl :: NoArg :: < { c :: FIONCLEX } > :: new () ; ioctl :: ioctl (fd , ctl) } }
};
}
