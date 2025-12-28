macro_rules! deps {
    () => {
        Result!();
        Getter!();
    };
}

macro_rules! ioctl_blkpbszget {
    () => {
        deps!();
        # [doc = " `ioctl(fd, BLKPBSZGET)`—Returns the physical block size of a block device."] # [cfg (linux_kernel)] # [inline] # [doc (alias = "BLKPBSZGET")] pub fn ioctl_blkpbszget < Fd : AsFd > (fd : Fd) -> io :: Result < u32 > { unsafe { let ctl = ioctl :: Getter :: < { c :: BLKPBSZGET } , c :: c_uint > :: new () ; ioctl :: ioctl (fd , ctl) } }
    };
}

ioctl_blkpbszget!()