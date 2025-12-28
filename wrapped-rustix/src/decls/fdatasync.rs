macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! fdatasync {
    () => {
        deps!();
        # [doc = " `fdatasync(fd)`—Ensures that file data is written to the underlying"] # [doc = " storage device."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/fdatasync.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/fdatasync.2.html"] # [cfg (not (any (apple , target_os = "dragonfly" , target_os = "espidf" , target_os = "haiku" , target_os = "horizon" , target_os = "redox" , target_os = "vita" ,)))] # [inline] pub fn fdatasync < Fd : AsFd > (fd : Fd) -> io :: Result < () > { backend :: fs :: syscalls :: fdatasync (fd . as_fd ()) }
    };
}

fdatasync!()