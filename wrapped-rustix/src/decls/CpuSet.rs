macro_rules! CpuSet {
    () => {
        # [doc = " `CpuSet` represents a bit-mask of CPUs."] # [doc = ""] # [doc = " `CpuSet`s are used by [`sched_setaffinity`] and [`sched_getaffinity`], for"] # [doc = " example."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [Linux]: https://man7.org/linux/man-pages/man3/CPU_SET.3.html"] # [doc = " [`sched_setaffinity`]: crate::thread::sched_setaffinity"] # [doc = " [`sched_getaffinity`]: crate::thread::sched_getaffinity"] # [repr (transparent)] # [derive (Clone , Copy)] pub struct CpuSet { cpu_set : backend :: thread :: types :: RawCpuSet , }
    };
}

CpuSet!();