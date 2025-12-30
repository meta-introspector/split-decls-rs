// Generated macro for impl_34 (impl)
macro_rules! Depcrate_network_configurationimpl_34 {
() => {
// Module: crate::network_configuration
// Provides: {"impl_34"}
// Dependencies: {}
impl SCNetworkSet { # [doc = " Constructs a new set of network services from the preferences."] pub fn new (prefs : & SCPreferences) -> Self { let ptr = unsafe { SCNetworkSetCopyCurrent (prefs . to_void ()) } ; unsafe { SCNetworkSet :: wrap_under_create_rule (ptr) } } # [doc = " Returns an list of network service identifiers, ordered by their priority."] pub fn service_order (& self) -> CFArray < CFString > { unsafe { let array_ptr = SCNetworkSetGetServiceOrder (self . 0) ; if array_ptr . is_null () { return create_empty_array () ; } CFArray :: < CFString > :: wrap_under_get_rule (array_ptr) } } }
};
}
