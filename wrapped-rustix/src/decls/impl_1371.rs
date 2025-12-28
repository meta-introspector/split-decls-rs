macro_rules! deps {
    () => {
        CpuSet!();
    };
}

macro_rules! impl_1371 {
    () => {
        deps!();
        impl CpuSet { # [doc = " The maximum number of CPU in `CpuSet`."] pub const MAX_CPU : usize = backend :: thread :: types :: CPU_SETSIZE ; # [doc = " Create a new and empty `CpuSet`."] # [inline] pub fn new () -> Self { Self { cpu_set : backend :: thread :: types :: raw_cpu_set_new () , } } # [doc = " Test to see if a CPU is in the `CpuSet`."] # [doc = ""] # [doc = " `field` is the CPU id to test."] # [inline] pub fn is_set (& self , field : usize) -> bool { backend :: thread :: cpu_set :: CPU_ISSET (field , & self . cpu_set) } # [doc = " Add a CPU to `CpuSet`."] # [doc = ""] # [doc = " `field` is the CPU id to add."] # [inline] pub fn set (& mut self , field : usize) { backend :: thread :: cpu_set :: CPU_SET (field , & mut self . cpu_set) } # [doc = " Remove a CPU from `CpuSet`."] # [doc = ""] # [doc = " `field` is the CPU id to remove."] # [inline] pub fn unset (& mut self , field : usize) { backend :: thread :: cpu_set :: CPU_CLR (field , & mut self . cpu_set) } # [doc = " Count the number of CPUs set in the `CpuSet`."] # [cfg (linux_kernel)] # [inline] pub fn count (& self) -> u32 { backend :: thread :: cpu_set :: CPU_COUNT (& self . cpu_set) } # [doc = " Zeroes the `CpuSet`."] # [inline] pub fn clear (& mut self) { backend :: thread :: cpu_set :: CPU_ZERO (& mut self . cpu_set) } }
    };
}

impl_1371!()