// Generated macro for get_interface_address (function)
macro_rules! Depcrate_unix_network_helperget_interface_address {
() => {
// Module: crate::unix::network_helper
// Provides: {"get_interface_address"}
// Dependencies: {}
# [doc = " Return an iterator on (interface_name, address) pairs"] pub (crate) unsafe fn get_interface_address () -> Result < InterfaceAddressIterator , String > { let mut ifap = null_mut () ; if unsafe { retry_eintr ! (libc :: getifaddrs (& mut ifap)) } == 0 && ! ifap . is_null () { Ok (InterfaceAddressIterator { ifap , buf : ifap }) } else { Err ("failed to call getifaddrs()" . to_string ()) } }
};
}
