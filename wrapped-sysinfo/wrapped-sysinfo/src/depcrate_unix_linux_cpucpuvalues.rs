// Generated macro for CpuValues (struct)
macro_rules! Depcrate_unix_linux_cpuCpuValues {
() => {
// Module: crate::unix::linux::cpu
// Provides: {"CpuValues"}
// Dependencies: {}
# [doc = " Struct containing values to compute a CPU usage."] # [derive (Clone , Copy , Debug , Default)] pub (crate) struct CpuValues { user : u64 , nice : u64 , system : u64 , idle : u64 , iowait : u64 , irq : u64 , softirq : u64 , steal : u64 , guest : u64 , guest_nice : u64 , }
};
}
