// Generated macro for tests (module)
macro_rules! Depcrate_sysinfotests {
() => {
// Module: crate::sysinfo
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn itworks () { let kinds = [ComputerNameKind :: DnsDomain , ComputerNameKind :: DnsFullyQualified , ComputerNameKind :: DnsHostname , ComputerNameKind :: NetBios , ComputerNameKind :: PhysicalDnsDomain , ComputerNameKind :: PhysicalDnsFullyQualified , ComputerNameKind :: PhysicalDnsHostname , ComputerNameKind :: PhysicalNetBios ,] ; for kind in kinds { let result = get_computer_name (kind) ; let name = result . unwrap () ; println ! ("{kind:?}: {name:?}") ; } } }
};
}
