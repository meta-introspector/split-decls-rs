// Generated macro for sysinfo_init (function)
macro_rules! Depcrate_c_interfacesysinfo_init {
() => {
// Module: crate::c_interface
// Provides: {"sysinfo_init"}
// Dependencies: {}
# [doc = " Equivalent of [`System::new()`][crate::System#method.new]."] # [unsafe (no_mangle)] pub extern "C" fn sysinfo_init () -> CSystem { let system = Box :: new (System :: new ()) ; Box :: into_raw (system) as CSystem }
};
}
