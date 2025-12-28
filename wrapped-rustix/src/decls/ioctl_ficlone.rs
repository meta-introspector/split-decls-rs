macro_rules! deps {
    () => {
        Ficlone!();
        Result!();
    };
}

macro_rules! ioctl_ficlone {
    () => {
        deps!();
        # [doc = " `ioctl(fd, FICLONE, src_fd)`—Share data between open files."] # [doc = ""] # [doc = " This ioctl is not available on SPARC platforms."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/ioctl_ficlone.2.html"] # [cfg (all (linux_kernel , not (any (target_arch = "sparc" , target_arch = "sparc64"))))] # [inline] # [doc (alias = "FICLONE")] pub fn ioctl_ficlone < Fd : AsFd , SrcFd : AsFd > (fd : Fd , src_fd : SrcFd) -> io :: Result < () > { unsafe { ioctl :: ioctl (fd , Ficlone (src_fd . as_fd ())) } }
    };
}

ioctl_ficlone!()