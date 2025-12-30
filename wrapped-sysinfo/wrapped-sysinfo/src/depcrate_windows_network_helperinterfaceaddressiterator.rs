// Generated macro for InterfaceAddressIterator (struct)
macro_rules! Depcrate_windows_network_helperInterfaceAddressIterator {
() => {
// Module: crate::windows::network_helper
// Provides: {"InterfaceAddressIterator"}
// Dependencies: {}
# [doc = " this iterator yields an interface name and address"] pub (crate) struct InterfaceAddressIterator { # [doc = " The first item in the linked list"] buf : * mut IP_ADAPTER_ADDRESSES_LH , # [doc = " The current adapter"] adapter : * mut IP_ADAPTER_ADDRESSES_LH , }
};
}
