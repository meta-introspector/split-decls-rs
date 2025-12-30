// Generated macro for init_cpus (function)
macro_rules! Depcrate_unix_apple_cpuinit_cpus {
() => {
// Module: crate::unix::apple::cpu
// Provides: {"init_cpus"}
// Dependencies: {}
pub (crate) fn init_cpus (port : libc :: mach_port_t , cpus : & mut Vec < Cpu > , global_cpu : & mut CpuUsage , refresh_kind : CpuRefreshKind ,) { let mut num_cpu = 0 ; let mut mib = [libc :: CTL_HW as _ , libc :: HW_NCPU as _] ; let (vendor_id , brand) = get_vendor_id_and_brand () ; let frequency = if refresh_kind . frequency () { unsafe { get_cpu_frequency (& brand) } } else { global_cpu . frequency } ; unsafe { if ! get_sys_value (mem :: size_of :: < u32 > () , & mut num_cpu as * mut _ as * mut _ , & mut mib ,) { num_cpu = 1 ; } } update_cpu_usage (port , global_cpu , | proc_data , cpu_info | { let mut percentage = 0f32 ; let mut offset = 0 ; for i in 0 .. num_cpu { let mut cpu = Cpu { inner : CpuInner :: new (format ! ("{}" , i + 1) , Arc :: clone (& proc_data) , frequency , vendor_id . clone () , brand . clone () ,) , } ; if refresh_kind . cpu_usage () { let cpu_usage = compute_usage_of_cpu (& cpu , cpu_info , offset) ; cpu . inner . set_cpu_usage (cpu_usage) ; percentage += cpu . cpu_usage () ; } cpus . push (cpu) ; offset += libc :: CPU_STATE_MAX as isize ; } (percentage , cpus . len ()) }) ; }
};
}
