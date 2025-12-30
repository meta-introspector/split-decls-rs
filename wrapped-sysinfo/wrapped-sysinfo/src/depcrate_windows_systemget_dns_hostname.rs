// Generated macro for get_dns_hostname (function)
macro_rules! Depcrate_windows_systemget_dns_hostname {
() => {
// Module: crate::windows::system
// Provides: {"get_dns_hostname"}
// Dependencies: {}
fn get_dns_hostname () -> Option < String > { let mut buffer_size = 0 ; unsafe { let _err = GetComputerNameExW (ComputerNamePhysicalDnsHostname , None , & mut buffer_size) ; let mut buffer = vec ! [0_u16 ; buffer_size as usize] ; if GetComputerNameExW (ComputerNamePhysicalDnsHostname , Some (PWSTR :: from_raw (buffer . as_mut_ptr ())) , & mut buffer_size ,) . is_ok () { if let Some (pos) = buffer . iter () . position (| c | * c == 0) { buffer . resize (pos , 0) ; } return String :: from_utf16 (& buffer) . ok () ; } } sysinfo_debug ! ("Failed to get computer hostname") ; None }
};
}
