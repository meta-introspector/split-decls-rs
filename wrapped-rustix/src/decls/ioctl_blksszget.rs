macro_rules! deps {
    () => {
        Getter!();
        Result!();
    };
}

macro_rules! ioctl_blksszget {
    () => {
        deps!();
        # [doc = " `ioctl(fd, BLKSSZGET)`—Returns the logical block size of a block device."] # [doc = ""] # [doc = " This is mentioned in the [Linux `openat` manual page]."] # [doc = ""] # [doc = " [Linux `openat` manual page]: https://man7.org/linux/man-pages/man2/openat.2.html"] # [cfg (linux_kernel)] # [inline] # [doc (alias = "BLKSSZGET")] pub fn ioctl_blksszget < Fd : AsFd > (fd : Fd) -> io :: Result < u32 > { unsafe { let ctl = ioctl :: Getter :: < { c :: BLKSSZGET } , c :: c_uint > :: new () ; ioctl :: ioctl (fd , ctl) } }
    };
}

ioctl_blksszget!();