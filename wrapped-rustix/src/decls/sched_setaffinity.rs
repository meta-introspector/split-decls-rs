macro_rules! deps {
    () => {
        CpuSet!();
        Result!();
        Pid!();
    };
}

macro_rules! sched_setaffinity {
    () => {
        deps!();
        # [doc = " `sched_setaffinity(pid, cpuset)`—Set a thread's CPU affinity mask."] # [doc = ""] # [doc = " `pid` is the thread ID to update. If pid is `None`, then the current thread"] # [doc = " is updated."] # [doc = ""] # [doc = " The `CpuSet` argument specifies the set of CPUs on which the thread will be"] # [doc = " eligible to run."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/sched_setaffinity.2.html"] # [inline] pub fn sched_setaffinity (pid : Option < Pid > , cpuset : & CpuSet) -> io :: Result < () > { backend :: thread :: syscalls :: sched_setaffinity (pid , & cpuset . cpu_set) }
    };
}

sched_setaffinity!()