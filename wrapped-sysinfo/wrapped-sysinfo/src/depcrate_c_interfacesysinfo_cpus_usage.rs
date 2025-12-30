// Generated macro for sysinfo_cpus_usage (function)
macro_rules! Depcrate_c_interfacesysinfo_cpus_usage {
() => {
// Module: crate::c_interface
// Provides: {"sysinfo_cpus_usage"}
// Dependencies: {}
# [doc = " Equivalent of [`System::cpus_usage()`][crate::System#method.cpus_usage]."] # [doc = ""] # [doc = " * `length` will contain the number of CPU usage added into `procs`."] # [doc = " * `procs` will be allocated if it's null and will contain of CPU usage."] # [unsafe (no_mangle)] pub extern "C" fn sysinfo_cpus_usage (system : CSystem , length : * mut c_uint , procs : * mut * mut c_float ,) { assert ! (! system . is_null ()) ; if procs . is_null () || length . is_null () { return ; } unsafe { let system : Box < System > = Box :: from_raw (system as * mut System) ; { let cpus = system . cpus () ; if (* procs) . is_null () { (* procs) = libc :: malloc (:: std :: mem :: size_of :: < c_float > () * cpus . len ()) as * mut c_float ; } for (pos , cpu) in cpus . iter () . skip (1) . enumerate () { (* (* procs) . offset (pos as isize)) = cpu . cpu_usage () ; } * length = cpus . len () as c_uint - 1 ; } let _ = Box :: into_raw (system) ; } }
};
}
