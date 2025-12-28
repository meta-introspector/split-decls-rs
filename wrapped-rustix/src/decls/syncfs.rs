macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! syncfs {
    () => {
        deps!();
        # [doc = " `syncfs(fd)`—Flush cached filesystem data."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/syncfs.2.html"] # [cfg (linux_kernel)] # [inline] pub fn syncfs < Fd : AsFd > (fd : Fd) -> io :: Result < () > { backend :: fs :: syscalls :: syncfs (fd . as_fd ()) }
    };
}

syncfs!();