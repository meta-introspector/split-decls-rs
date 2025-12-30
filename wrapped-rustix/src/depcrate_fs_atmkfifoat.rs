// Generated macro for mkfifoat (function)
macro_rules! Depcrate_fs_atmkfifoat {
() => {
// Module: crate::fs::at
// Provides: {"mkfifoat"}
// Dependencies: {}
# [doc = " `mkfifoat(dirfd, path, mode)`—Make a FIFO special file."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = ""] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/mkfifoat.html"] # [cfg (not (any (apple , target_os = "espidf" , target_os = "horizon" , target_os = "vita" , target_os = "wasi")))] # [inline] pub fn mkfifoat < P : path :: Arg , Fd : AsFd > (dirfd : Fd , path : P , mode : Mode) -> io :: Result < () > { mknodat (dirfd , path , FileType :: Fifo , mode , 0) }
};
}
