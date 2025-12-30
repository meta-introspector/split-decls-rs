// Generated macro for impl_1170 (impl)
macro_rules! Depcrate_windows_network_helperimpl_1170 {
() => {
// Module: crate::windows::network_helper
// Provides: {"impl_1170"}
// Dependencies: {}
impl InterfaceAddressIterator { pub fn generate_ip_networks (& mut self) -> HashMap < String , HashSet < IpNetwork > > { let mut results = HashMap :: new () ; while ! self . adapter . is_null () { unsafe { let adapter = self . adapter ; self . adapter = (* adapter) . Next ; if let Ok (interface_name) = (* adapter) . FriendlyName . to_string () { let ip_networks = get_ip_networks ((* adapter) . FirstUnicastAddress) ; results . insert (interface_name , ip_networks) ; } } } results } }
};
}
