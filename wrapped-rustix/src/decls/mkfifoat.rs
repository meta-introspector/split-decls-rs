macro_rules! deps {
    () => {
        Result!();
        Arg!();
    };
}

macro_rules! mkfifoat {
    () => {
        deps!();
        # [doc = " `mkfifoat(dirfd, path, mode)`—Make a FIFO special file."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = ""] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/mkfifoat.html"] # [cfg (not (any (apple , target_os = "espidf" , target_os = "horizon" , target_os = "vita" , target_os = "wasi")))] # [inline] pub fn mkfifoat < P : path :: Arg , Fd : AsFd > (dirfd : Fd , path : P , mode : Mode) -> io :: Result < () > { mknodat (dirfd , path , FileType :: Fifo , mode , 0) }
    };
}

mkfifoat!();