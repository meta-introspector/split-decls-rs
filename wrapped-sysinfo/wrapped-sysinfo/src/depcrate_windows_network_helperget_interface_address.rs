// Generated macro for get_interface_address (function)
macro_rules! Depcrate_windows_network_helperget_interface_address {
() => {
// Module: crate::windows::network_helper
// Provides: {"get_interface_address"}
// Dependencies: {}
pub (crate) unsafe fn get_interface_address () -> Result < InterfaceAddressIterator , String > { let mut size : u32 = 15 * 1024 ; let mut ret = ERROR_SUCCESS . 0 ; let mut iterator = InterfaceAddressIterator :: new () ; for _ in 0 .. 3 { unsafe { iterator = iterator . realloc (size as _) ? ; ret = GetAdaptersAddresses (AF_UNSPEC . 0 . into () , GAA_FLAG_SKIP_MULTICAST | GAA_FLAG_SKIP_ANYCAST | GAA_FLAG_SKIP_DNS_SERVER , None , Some (iterator . buf) , & mut size ,) ; if ret == ERROR_SUCCESS . 0 { return Ok (iterator) ; } else if ret != ERROR_BUFFER_OVERFLOW . 0 { break ; } } } Err (format ! ("GetAdaptersAddresses() failed with code {ret}")) }
};
}
