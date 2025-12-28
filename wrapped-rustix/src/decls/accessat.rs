macro_rules! deps {
    () => {
        Arg!();
        Result!();
    };
}

macro_rules! accessat {
    () => {
        deps!();
        # [doc = " `faccessat(dirfd, path, access, flags)`—Tests permissions for a file or"] # [doc = " directory."] # [doc = ""] # [doc = " On Linux before 5.8, this function uses the `faccessat` system call which"] # [doc = " doesn't support any flags. This function emulates support for the"] # [doc = " [`AtFlags::EACCESS`] flag by checking whether the uid and gid of the"] # [doc = " process match the effective uid and gid, in which case the `EACCESS` flag"] # [doc = " can be ignored. In Linux 5.8 and beyond `faccessat2` is used, which"] # [doc = " supports flags."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/faccessat.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/faccessat.2.html"] # [cfg (not (any (target_os = "espidf" , target_os = "horizon" , target_os = "vita")))] # [inline] # [doc (alias = "faccessat")] pub fn accessat < P : path :: Arg , Fd : AsFd > (dirfd : Fd , path : P , access : Access , flags : AtFlags ,) -> io :: Result < () > { path . into_with_c_str (| path | backend :: fs :: syscalls :: accessat (dirfd . as_fd () , path , access , flags)) }
    };
}

accessat!();