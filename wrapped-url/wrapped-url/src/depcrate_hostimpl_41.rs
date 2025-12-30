// Generated macro for impl_41 (impl)
macro_rules! Depcrate_hostimpl_41 {
() => {
// Module: crate::host
// Provides: {"impl_41"}
// Dependencies: {}
impl From < Host < Cow < '_ , str > > > for HostInternal { fn from (host : Host < Cow < '_ , str > >) -> Self { match host { Host :: Domain (ref s) if s . is_empty () => Self :: None , Host :: Domain (_) => Self :: Domain , Host :: Ipv4 (address) => Self :: Ipv4 (address) , Host :: Ipv6 (address) => Self :: Ipv6 (address) , } } }
};
}
