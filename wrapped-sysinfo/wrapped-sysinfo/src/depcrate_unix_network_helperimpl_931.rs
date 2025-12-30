// Generated macro for impl_931 (impl)
macro_rules! Depcrate_unix_network_helperimpl_931 {
() => {
// Module: crate::unix::network_helper
// Provides: {"impl_931"}
// Dependencies: {}
impl Iterator for InterfaceAddressIterator { type Item = (String , MacAddr) ; fn next (& mut self) -> Option < Self :: Item > { unsafe { while ! self . ifap . is_null () { let ifap = & * self . ifap ; self . ifap = ifap . ifa_next ; if let Some (addr) = parse_interface_address (ifap) { let ifa_name = ifap . ifa_name ; if ifa_name . is_null () { continue ; } let mut name = vec ! [0u8 ; libc :: IFNAMSIZ + 6] ; libc :: strcpy (name . as_mut_ptr () as _ , ifap . ifa_name) ; name . set_len (libc :: strlen (ifap . ifa_name)) ; let name = String :: from_utf8_unchecked (name) ; return Some ((name , addr)) ; } } None } } }
};
}
