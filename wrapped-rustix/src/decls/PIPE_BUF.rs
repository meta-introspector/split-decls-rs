macro_rules! PIPE_BUF {
    () => {
        # [doc = " `PIPE_BUF`—The maximum length at which writes to a pipe are atomic."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux]"] # [doc = "  - [POSIX]"] # [doc = ""] # [doc = " [Linux]: https://man7.org/linux/man-pages/man7/pipe.7.html"] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/write.html"] # [cfg (not (any (solarish , windows , target_os = "espidf" , target_os = "haiku" , target_os = "horizon" , target_os = "hurd" , target_os = "redox" , target_os = "vita" , target_os = "wasi" ,)))] pub const PIPE_BUF : usize = c :: PIPE_BUF ;
    };
}

PIPE_BUF!()