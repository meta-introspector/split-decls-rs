macro_rules! deps {
    () => {
        Result!();
        CpuSet!();
        Pid!();
    };
}

macro_rules! sched_getaffinity {
    () => {
        deps!();
        # [doc = " `sched_getaffinity(pid)`—Get a thread's CPU affinity mask."] # [doc = ""] # [doc = " `pid` is the thread ID to check. If pid is `None`, then the current thread"] # [doc = " is checked."] # [doc = ""] # [doc = " Returns the set of CPUs on which the thread is eligible to run."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/sched_getaffinity.2.html"] # [inline] pub fn sched_getaffinity (pid : Option < Pid >) -> io :: Result < CpuSet > { let mut cpuset = CpuSet :: new () ; backend :: thread :: syscalls :: sched_getaffinity (pid , & mut cpuset . cpu_set) . and (Ok (cpuset)) }
    };
}

sched_getaffinity!()