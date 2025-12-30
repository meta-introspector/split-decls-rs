// Generated macro for physical_core_count (function)
macro_rules! Depcrate_unix_apple_cpuphysical_core_count {
() => {
// Module: crate::unix::apple::cpu
// Provides: {"physical_core_count"}
// Dependencies: {}
pub (crate) fn physical_core_count () -> Option < usize > { let mut physical_core_count = 0 ; unsafe { if get_sys_value_by_name (b"hw.physicalcpu\0" , & mut mem :: size_of :: < u32 > () , & mut physical_core_count as * mut usize as * mut c_void ,) { Some (physical_core_count) } else { None } } }
};
}
