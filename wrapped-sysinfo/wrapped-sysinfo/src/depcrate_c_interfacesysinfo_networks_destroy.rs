// Generated macro for sysinfo_networks_destroy (function)
macro_rules! Depcrate_c_interfacesysinfo_networks_destroy {
() => {
// Module: crate::c_interface
// Provides: {"sysinfo_networks_destroy"}
// Dependencies: {}
# [doc = " Equivalent of `Networks::drop()`. Important in C to cleanup memory."] # [unsafe (no_mangle)] pub extern "C" fn sysinfo_networks_destroy (networks : CNetworks) { assert ! (! networks . is_null ()) ; unsafe { drop (Box :: from_raw (networks as * mut Networks)) ; } }
};
}
