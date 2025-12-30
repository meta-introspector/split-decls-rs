// Generated macro for impl_31 (impl)
macro_rules! Depcrate_network_configurationimpl_31 {
() => {
// Module: crate::network_configuration
// Provides: {"impl_31"}
// Dependencies: {}
impl SCNetworkService { # [doc = " Returns an array of all network services"] pub fn get_services (prefs : & SCPreferences) -> CFArray < Self > { unsafe { let array_ptr = SCNetworkServiceCopyAll (prefs . to_void ()) ; if array_ptr . is_null () { return create_empty_array () ; } CFArray :: < Self > :: wrap_under_create_rule (array_ptr) } } # [doc = " Returns true if the network service is currently enabled"] pub fn enabled (& self) -> bool { unsafe { SCNetworkServiceGetEnabled (self . 0) == 0 } } # [doc = " Returns the network interface backing this network service, if it has one."] pub fn network_interface (& self) -> Option < SCNetworkInterface > { unsafe { let ptr = SCNetworkServiceGetInterface (self . 0) ; if ptr . is_null () { None } else { Some (SCNetworkInterface :: wrap_under_get_rule (ptr)) } } } # [doc = " Returns the service identifier."] pub fn id (& self) -> Option < CFString > { unsafe { let ptr = SCNetworkServiceGetServiceID (self . 0) ; if ptr . is_null () { None } else { Some (CFString :: wrap_under_get_rule (ptr)) } } } }
};
}
