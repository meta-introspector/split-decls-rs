macro_rules! deps {
    () => {
        Uid!();
        Result!();
        Arg!();
        Gid!();
    };
}

macro_rules! chown {
    () => {
        deps!();
        # [doc = " `chown(path, owner, group)`—Sets open file or directory ownership."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/chown.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/chown.2.html"] # [cfg (not (target_os = "wasi"))] # [inline] pub fn chown < P : path :: Arg > (path : P , owner : Option < Uid > , group : Option < Gid >) -> io :: Result < () > { path . into_with_c_str (| path | backend :: fs :: syscalls :: chown (path , owner , group)) }
    };
}

chown!()