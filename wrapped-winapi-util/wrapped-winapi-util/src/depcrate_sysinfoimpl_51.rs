// Generated macro for impl_51 (impl)
macro_rules! Depcrate_sysinfoimpl_51 {
() => {
// Module: crate::sysinfo
// Provides: {"impl_51"}
// Dependencies: {}
impl ComputerNameKind { fn to_format (& self) -> COMPUTER_NAME_FORMAT { use self :: ComputerNameKind :: * ; use windows_sys :: Win32 :: System :: SystemInformation ; match * self { DnsDomain => SystemInformation :: ComputerNameDnsDomain , DnsFullyQualified => { SystemInformation :: ComputerNameDnsFullyQualified } DnsHostname => SystemInformation :: ComputerNameDnsHostname , NetBios => SystemInformation :: ComputerNameNetBIOS , PhysicalDnsDomain => { SystemInformation :: ComputerNamePhysicalDnsDomain } PhysicalDnsFullyQualified => { SystemInformation :: ComputerNamePhysicalDnsFullyQualified } PhysicalDnsHostname => { SystemInformation :: ComputerNamePhysicalDnsHostname } PhysicalNetBios => SystemInformation :: ComputerNamePhysicalNetBIOS , } } }
};
}
