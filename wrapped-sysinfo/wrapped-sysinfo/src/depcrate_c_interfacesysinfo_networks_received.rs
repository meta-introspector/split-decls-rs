// Generated macro for sysinfo_networks_received (function)
macro_rules! Depcrate_c_interfacesysinfo_networks_received {
() => {
// Module: crate::c_interface
// Provides: {"sysinfo_networks_received"}
// Dependencies: {}
# [doc = " Equivalent of"] # [doc = " `system::networks().iter().fold(0, |acc, (_, data)| acc + data.received() as size_t)`."] # [unsafe (no_mangle)] pub extern "C" fn sysinfo_networks_received (networks : CNetworks) -> size_t { assert ! (! networks . is_null ()) ; unsafe { let networks : Box < Networks > = Box :: from_raw (networks as * mut Networks) ; let ret = networks . iter () . fold (0 , | acc : size_t , (_ , data) | { acc . saturating_add (data . received () as size_t) }) ; let _ = Box :: into_raw (networks) ; ret } }
};
}
