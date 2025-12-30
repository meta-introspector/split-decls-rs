// Generated macro for get_frequency_for_cpu (function)
macro_rules! Depcrate_unix_freebsd_cpuget_frequency_for_cpu {
() => {
// Module: crate::unix::freebsd::cpu
// Provides: {"get_frequency_for_cpu"}
// Dependencies: {}
unsafe fn get_frequency_for_cpu (cpu_nb : usize) -> u64 { let mut frequency : c_int = 0 ; unsafe { if ! get_sys_value_by_name (format ! ("dev.cpu.{cpu_nb}.freq\0") . as_bytes () , & mut frequency ,) { frequency = 0 ; } } frequency as _ }
};
}
