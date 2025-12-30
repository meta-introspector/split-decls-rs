// Generated macro for impl_1041 (impl)
macro_rules! Depcrate_net_sockoptimpl_1041 {
() => {
// Module: crate::net::sockopt
// Provides: {"impl_1041"}
// Dependencies: {}
# [cfg (linux_kernel)] impl Ipv4PathMtuDiscovery { # [doc = " `IP_PMTUDISC_DONT`"] # [doc (alias = "IP_PMTUDISC_DONT")] pub const DONT : Self = Self (c :: IP_PMTUDISC_DONT as _) ; # [doc = " `IP_PMTUDISC_WANT`"] # [doc (alias = "IP_PMTUDISC_WANT")] pub const WANT : Self = Self (c :: IP_PMTUDISC_WANT as _) ; # [doc = " `IP_PMTUDISC_DO`"] # [doc (alias = "IP_PMTUDISC_DO")] pub const DO : Self = Self (c :: IP_PMTUDISC_DO as _) ; # [doc = " `IP_PMTUDISC_PROBE`"] # [doc (alias = "IP_PMTUDISC_PROBE")] pub const PROBE : Self = Self (c :: IP_PMTUDISC_PROBE as _) ; # [doc = " `IP_PMTUDISC_INTERFACE`"] # [doc (alias = "IP_PMTUDISC_INTERFACE")] pub const INTERFACE : Self = Self (c :: IP_PMTUDISC_INTERFACE as _) ; # [doc = " `IP_PMTUDISC_OMIT`"] # [doc (alias = "IP_PMTUDISC_OMIT")] pub const OMIT : Self = Self (c :: IP_PMTUDISC_OMIT as _) ; # [doc = " Constructs an option from a raw integer."] # [inline] pub const fn from_raw (raw : RawIpv4PathMtuDiscovery) -> Self { Self (raw) } # [doc = " Returns the raw integer for this option."] # [inline] pub const fn as_raw (self) -> RawIpv4PathMtuDiscovery { self . 0 } }
};
}
