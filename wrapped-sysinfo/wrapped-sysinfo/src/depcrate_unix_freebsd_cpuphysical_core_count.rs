// Generated macro for physical_core_count (function)
macro_rules! Depcrate_unix_freebsd_cpuphysical_core_count {
() => {
// Module: crate::unix::freebsd::cpu
// Provides: {"physical_core_count"}
// Dependencies: {}
pub (crate) fn physical_core_count () -> Option < usize > { let mut physical_core_count : u32 = 0 ; unsafe { if get_sys_value_by_name (b"hw.ncpu\0" , & mut physical_core_count) { Some (physical_core_count as _) } else { None } } }
};
}
