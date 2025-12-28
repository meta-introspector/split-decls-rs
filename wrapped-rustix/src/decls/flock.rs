macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! flock {
    () => {
        deps!();
        # [doc = " `flock(fd, operation)`—Acquire or release an advisory lock on an open file."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/flock.2.html"] # [cfg (not (any (target_os = "espidf" , target_os = "horizon" , target_os = "solaris" , target_os = "vita" , target_os = "wasi")))] # [inline] pub fn flock < Fd : AsFd > (fd : Fd , operation : FlockOperation) -> io :: Result < () > { backend :: fs :: syscalls :: flock (fd . as_fd () , operation) }
    };
}

flock!()