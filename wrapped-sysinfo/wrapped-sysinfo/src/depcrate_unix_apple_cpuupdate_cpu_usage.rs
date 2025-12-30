// Generated macro for update_cpu_usage (function)
macro_rules! Depcrate_unix_apple_cpuupdate_cpu_usage {
() => {
// Module: crate::unix::apple::cpu
// Provides: {"update_cpu_usage"}
// Dependencies: {}
pub (crate) fn update_cpu_usage < F : FnOnce (Arc < CpuData > , * mut i32) -> (f32 , usize) > (port : libc :: mach_port_t , global_cpu : & mut CpuUsage , f : F ,) { let mut num_cpu_u = 0u32 ; let mut cpu_info : * mut i32 = std :: ptr :: null_mut () ; let mut num_cpu_info = 0u32 ; let mut total_cpu_usage = 0f32 ; unsafe { if host_processor_info (port , libc :: PROCESSOR_CPU_LOAD_INFO , & mut num_cpu_u as * mut u32 , & mut cpu_info as * mut * mut i32 , & mut num_cpu_info as * mut u32 ,) == libc :: KERN_SUCCESS { let (total_percentage , len) = f (Arc :: new (CpuData :: new (cpu_info , num_cpu_info)) , cpu_info) ; total_cpu_usage = total_percentage / len as f32 ; } global_cpu . set_cpu_usage (total_cpu_usage) ; } }
};
}
