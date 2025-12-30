// Generated macro for impl_1169 (impl)
macro_rules! Depcrate_windows_network_helperimpl_1169 {
() => {
// Module: crate::windows::network_helper
// Provides: {"impl_1169"}
// Dependencies: {}
impl Iterator for InterfaceAddressIterator { type Item = (String , MacAddr) ; fn next (& mut self) -> Option < Self :: Item > { if self . adapter . is_null () { return None ; } unsafe { let adapter = self . adapter ; self . adapter = (* adapter) . Next ; if let Ok (interface_name) = (* adapter) . FriendlyName . to_string () { let [mac @ .. , _ , _] = (* adapter) . PhysicalAddress ; Some ((interface_name , MacAddr (mac))) } else { self . next () } } } }
};
}
