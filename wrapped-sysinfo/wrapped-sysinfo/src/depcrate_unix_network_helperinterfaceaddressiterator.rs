// Generated macro for InterfaceAddressIterator (struct)
macro_rules! Depcrate_unix_network_helperInterfaceAddressIterator {
() => {
// Module: crate::unix::network_helper
// Provides: {"InterfaceAddressIterator"}
// Dependencies: {}
# [doc = " This iterator yields an interface name and address."] pub (crate) struct InterfaceAddressIterator { # [doc = " Pointer to the current `ifaddrs` struct."] ifap : * mut libc :: ifaddrs , # [doc = " Pointer to the first element in linked list."] buf : * mut libc :: ifaddrs , }
};
}
