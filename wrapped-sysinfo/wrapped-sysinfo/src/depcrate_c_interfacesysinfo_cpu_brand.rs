// Generated macro for sysinfo_cpu_brand (function)
macro_rules! Depcrate_c_interfacesysinfo_cpu_brand {
() => {
// Module: crate::c_interface
// Provides: {"sysinfo_cpu_brand"}
// Dependencies: {}
# [doc = " Equivalent of [`cpu::brand()`]."] # [unsafe (no_mangle)] pub extern "C" fn sysinfo_cpu_brand (system : CSystem) -> RString { assert ! (! system . is_null ()) ; unsafe { let system : Box < System > = Box :: from_raw (system as * mut System) ; let c_string = if let Some (c) = system . cpus () . first () . and_then (| cpu | CString :: new (cpu . brand ()) . ok ()) { c . into_raw () as RString } else { std :: ptr :: null () } ; let _ = Box :: into_raw (system) ; c_string } }
};
}
