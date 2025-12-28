macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! fcntl_lock {
    () => {
        deps!();
        # [doc = " `fcntl(fd, F_SETLK)`—Acquire or release an `fcntl`-style lock."] # [doc = ""] # [doc = " This function doesn't currently have an offset or len; it currently always"] # [doc = " sets the `l_len` field to 0, which is a special case that means the entire"] # [doc = " file should be locked."] # [doc = ""] # [doc = " Unlike `flock`-style locks, `fcntl`-style locks are process-associated,"] # [doc = " meaning that they don't guard against being acquired by two threads in the"] # [doc = " same process."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/fcntl.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/fcntl.2.html"] # [cfg (not (any (target_os = "emscripten" , target_os = "espidf" , target_os = "fuchsia" , target_os = "horizon" , target_os = "redox" , target_os = "vita" , target_os = "wasi")))] # [inline] # [doc (alias = "F_SETLK")] # [doc (alias = "F_SETLKW")] pub fn fcntl_lock < Fd : AsFd > (fd : Fd , operation : FlockOperation) -> io :: Result < () > { backend :: fs :: syscalls :: fcntl_lock (fd . as_fd () , operation) }
    };
}

fcntl_lock!()