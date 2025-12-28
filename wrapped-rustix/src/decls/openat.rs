macro_rules! deps {
    () => {
        Result!();
        Arg!();
    };
}

macro_rules! openat {
    () => {
        deps!();
        # [doc = " `openat(dirfd, path, oflags, mode)`—Opens a file."] # [doc = ""] # [doc = " POSIX guarantees that `openat` will use the lowest unused file descriptor,"] # [doc = " however it is not safe in general to rely on this, as file descriptors may"] # [doc = " be unexpectedly allocated on other threads or in libraries."] # [doc = ""] # [doc = " The `Mode` argument is only significant when creating a file."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/openat.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/openat.2.html"] # [inline] pub fn openat < P : path :: Arg , Fd : AsFd > (dirfd : Fd , path : P , oflags : OFlags , create_mode : Mode ,) -> io :: Result < OwnedFd > { path . into_with_c_str (| path | { backend :: fs :: syscalls :: openat (dirfd . as_fd () , path , oflags , create_mode) }) }
    };
}

openat!()