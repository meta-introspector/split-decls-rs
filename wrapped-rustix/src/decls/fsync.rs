macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! fsync {
    () => {
        deps!();
        # [doc = " `fsync(fd)`—Ensures that file data and metadata is written to the"] # [doc = " underlying storage device."] # [doc = ""] # [doc = " On iOS and macOS this isn't sufficient to ensure that data has reached"] # [doc = " persistent storage; use [`fcntl_fullfsync`] to ensure that."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/fsync.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/fsync.2.html"] # [doc = " [`fcntl_fullfsync`]: https://docs.rs/rustix/*/x86_64-apple-darwin/rustix/fs/fn.fcntl_fullfsync.html"] # [inline] pub fn fsync < Fd : AsFd > (fd : Fd) -> io :: Result < () > { backend :: fs :: syscalls :: fsync (fd . as_fd ()) }
    };
}

fsync!();