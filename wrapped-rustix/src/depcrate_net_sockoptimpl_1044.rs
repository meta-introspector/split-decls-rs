// Generated macro for impl_1044 (impl)
macro_rules! Depcrate_net_sockoptimpl_1044 {
() => {
// Module: crate::net::sockopt
// Provides: {"impl_1044"}
// Dependencies: {}
# [cfg (linux_kernel)] impl Ipv6PathMtuDiscovery { # [doc = " `IPV6_PMTUDISC_DONT`"] # [doc (alias = "IPV6_PMTUDISC_DONT")] pub const DONT : Self = Self (c :: IPV6_PMTUDISC_DONT as _) ; # [doc = " `IPV6_PMTUDISC_WANT`"] # [doc (alias = "IPV6_PMTUDISC_WANT")] pub const WANT : Self = Self (c :: IPV6_PMTUDISC_WANT as _) ; # [doc = " `IPV6_PMTUDISC_DO`"] # [doc (alias = "IPV6_PMTUDISC_DO")] pub const DO : Self = Self (c :: IPV6_PMTUDISC_DO as _) ; # [doc = " `IPV6_PMTUDISC_PROBE`"] # [doc (alias = "IPV6_PMTUDISC_PROBE")] pub const PROBE : Self = Self (c :: IPV6_PMTUDISC_PROBE as _) ; # [doc = " `IPV6_PMTUDISC_INTERFACE`"] # [doc (alias = "IPV6_PMTUDISC_INTERFACE")] pub const INTERFACE : Self = Self (c :: IPV6_PMTUDISC_INTERFACE as _) ; # [doc = " `IPV6_PMTUDISC_OMIT`"] # [doc (alias = "IPV6_PMTUDISC_OMIT")] pub const OMIT : Self = Self (c :: IPV6_PMTUDISC_OMIT as _) ; # [doc = " Constructs an option from a raw integer."] # [inline] pub const fn from_raw (raw : RawIpv6PathMtuDiscovery) -> Self { Self (raw) } # [doc = " Returns the raw integer for this option."] # [inline] pub const fn as_raw (self) -> RawIpv6PathMtuDiscovery { self . 0 } }
};
}
