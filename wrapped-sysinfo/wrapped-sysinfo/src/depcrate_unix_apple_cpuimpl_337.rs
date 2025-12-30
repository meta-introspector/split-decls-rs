// Generated macro for impl_337 (impl)
macro_rules! Depcrate_unix_apple_cpuimpl_337 {
() => {
// Module: crate::unix::apple::cpu
// Provides: {"impl_337"}
// Dependencies: {}
impl Drop for CpuData { fn drop (& mut self) { if ! self . cpu_info . 0 . is_null () { let prev_cpu_info_size = std :: mem :: size_of :: < i32 > () as u32 * self . num_cpu_info ; unsafe { libc :: vm_deallocate (# [allow (deprecated)] mach_task_self () , self . cpu_info . 0 as _ , prev_cpu_info_size as _ ,) ; } self . cpu_info . 0 = std :: ptr :: null_mut () ; } } }
};
}
