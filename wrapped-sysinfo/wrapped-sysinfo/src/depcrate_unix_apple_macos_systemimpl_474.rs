// Generated macro for impl_474 (impl)
macro_rules! Depcrate_unix_apple_macos_systemimpl_474 {
() => {
// Module: crate::unix::apple::macos::system
// Provides: {"impl_474"}
// Dependencies: {}
impl ProcessorCpuLoadInfo { fn new (port : mach_port_t) -> Option < Self > { let mut info_size = std :: mem :: size_of :: < processor_cpu_load_info_t > () as _ ; let mut cpu_count = 0 ; let mut cpu_load : processor_cpu_load_info_t = null_mut () ; unsafe { if host_processor_info (port , PROCESSOR_CPU_LOAD_INFO , & mut cpu_count , & mut cpu_load as * mut _ as * mut _ , & mut info_size ,) != 0 { sysinfo_debug ! ("host_processor_info failed, not updating CPU ticks usage...") ; None } else if cpu_count < 1 || cpu_load . is_null () { None } else { Some (Self { cpu_load , cpu_count , }) } } } }
};
}
