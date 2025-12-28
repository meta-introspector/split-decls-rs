macro_rules! deps {
    () => {
        Pid!();
        Result!();
        Rlimit!();
    };
}

macro_rules! prlimit {
    () => {
        deps!();
        # [doc = " `prlimit(pid, resource, new)`—Get and set a process resource limit value."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/prlimit.2.html"] # [cfg (linux_kernel)] # [inline] pub fn prlimit (pid : Option < Pid > , resource : Resource , new : Rlimit) -> io :: Result < Rlimit > { backend :: process :: syscalls :: prlimit (pid , resource , new) }
    };
}

prlimit!();