macro_rules! deps {
    () => {
        Getter!();
        Result!();
    };
}

macro_rules! ioctl_getflags {
    () => {
        deps!();
        # [doc = " `ioctl(fd, FS_IOC_GETFLAGS)`—Returns the [inode flags] attributes"] # [doc = ""] # [doc = " [inode flags]: https://man7.org/linux/man-pages/man2/ioctl_iflags.2.html"] # [cfg (linux_raw_dep)] # [inline] # [doc (alias = "FS_IOC_GETFLAGS")] pub fn ioctl_getflags < Fd : AsFd > (fd : Fd) -> io :: Result < IFlags > { unsafe { # [cfg (target_pointer_width = "32")] let ctl = ioctl :: Getter :: < { c :: FS_IOC32_GETFLAGS } , u32 > :: new () ; # [cfg (target_pointer_width = "64")] let ctl = ioctl :: Getter :: < { c :: FS_IOC_GETFLAGS } , u32 > :: new () ; ioctl :: ioctl (fd , ctl) . map (IFlags :: from_bits_retain) } }
    };
}

ioctl_getflags!()