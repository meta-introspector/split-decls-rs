macro_rules! deps {
    () => {
        Setter!();
        Result!();
    };
}

macro_rules! ioctl_setflags {
    () => {
        deps!();
        # [doc = " `ioctl(fd, FS_IOC_SETFLAGS)`—Modify the [inode flags] attributes"] # [doc = ""] # [doc = " [inode flags]: https://man7.org/linux/man-pages/man2/ioctl_iflags.2.html"] # [cfg (linux_raw_dep)] # [inline] # [doc (alias = "FS_IOC_SETFLAGS")] pub fn ioctl_setflags < Fd : AsFd > (fd : Fd , flags : IFlags) -> io :: Result < () > { unsafe { # [cfg (target_pointer_width = "32")] let ctl = ioctl :: Setter :: < { c :: FS_IOC32_SETFLAGS } , u32 > :: new (flags . bits ()) ; # [cfg (target_pointer_width = "64")] let ctl = ioctl :: Setter :: < { c :: FS_IOC_SETFLAGS } , u32 > :: new (flags . bits ()) ; ioctl :: ioctl (fd , ctl) } }
    };
}

ioctl_setflags!();