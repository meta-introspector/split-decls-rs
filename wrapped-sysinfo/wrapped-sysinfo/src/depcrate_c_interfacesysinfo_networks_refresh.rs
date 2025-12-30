// Generated macro for sysinfo_networks_refresh (function)
macro_rules! Depcrate_c_interfacesysinfo_networks_refresh {
() => {
// Module: crate::c_interface
// Provides: {"sysinfo_networks_refresh"}
// Dependencies: {}
# [doc = " Equivalent of [`Networks::refresh()`][crate::Networks#method.refresh]."] # [unsafe (no_mangle)] pub extern "C" fn sysinfo_networks_refresh (networks : CNetworks) { assert ! (! networks . is_null ()) ; unsafe { let mut networks : Box < Networks > = Box :: from_raw (networks as * mut Networks) ; { let networks : & mut Networks = networks . borrow_mut () ; networks . refresh (true) ; } let _ = Box :: into_raw (networks) ; } }
};
}
