// Generated macro for sysinfo_networks_init (function)
macro_rules! Depcrate_c_interfacesysinfo_networks_init {
() => {
// Module: crate::c_interface
// Provides: {"sysinfo_networks_init"}
// Dependencies: {}
# [doc = " Equivalent of [`Networks::new()`][crate::Networks#method.new]."] # [unsafe (no_mangle)] pub extern "C" fn sysinfo_networks_init () -> CNetworks { let networks = Box :: new (Networks :: new ()) ; Box :: into_raw (networks) as CNetworks }
};
}
