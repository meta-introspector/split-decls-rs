// Generated macro for sysinfo_cpu_physical_cores (function)
macro_rules! Depcrate_c_interfacesysinfo_cpu_physical_cores {
() => {
// Module: crate::c_interface
// Provides: {"sysinfo_cpu_physical_cores"}
// Dependencies: {}
# [doc = " Equivalent of [`system::physical_core_count()`]."] # [unsafe (no_mangle)] pub extern "C" fn sysinfo_cpu_physical_cores () -> u32 { System :: physical_core_count () . unwrap_or (0) as u32 }
};
}
