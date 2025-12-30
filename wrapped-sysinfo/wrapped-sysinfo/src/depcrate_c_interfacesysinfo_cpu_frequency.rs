// Generated macro for sysinfo_cpu_frequency (function)
macro_rules! Depcrate_c_interfacesysinfo_cpu_frequency {
() => {
// Module: crate::c_interface
// Provides: {"sysinfo_cpu_frequency"}
// Dependencies: {}
# [doc = " Equivalent of [`cpu::frequency()`]."] # [unsafe (no_mangle)] pub extern "C" fn sysinfo_cpu_frequency (system : CSystem) -> u64 { assert ! (! system . is_null ()) ; unsafe { let system : Box < System > = Box :: from_raw (system as * mut System) ; let freq = system . cpus () . first () . map (| cpu | cpu . frequency ()) . unwrap_or (0) ; let _ = Box :: into_raw (system) ; freq } }
};
}
