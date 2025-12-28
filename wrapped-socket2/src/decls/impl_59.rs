macro_rules! deps {
    () => {
        Protocol!();
    };
}

macro_rules! impl_59 {
    () => {
        deps!();
        impl Protocol { # [doc = " Protocol corresponding to `ICMPv4`."] pub const ICMPV4 : Protocol = Protocol (sys :: IPPROTO_ICMP) ; # [doc = " Protocol corresponding to `ICMPv6`."] pub const ICMPV6 : Protocol = Protocol (sys :: IPPROTO_ICMPV6) ; # [doc = " Protocol corresponding to `TCP`."] pub const TCP : Protocol = Protocol (sys :: IPPROTO_TCP) ; # [doc = " Protocol corresponding to `UDP`."] pub const UDP : Protocol = Protocol (sys :: IPPROTO_UDP) ; # [cfg (target_os = "linux")] # [doc = " Protocol corresponding to `MPTCP`."] pub const MPTCP : Protocol = Protocol (sys :: IPPROTO_MPTCP) ; # [doc = " Protocol corresponding to `DCCP`."] # [cfg (all (feature = "all" , target_os = "linux"))] pub const DCCP : Protocol = Protocol (sys :: IPPROTO_DCCP) ; # [doc = " Protocol corresponding to `SCTP`."] # [cfg (all (feature = "all" , any (target_os = "freebsd" , target_os = "linux")))] pub const SCTP : Protocol = Protocol (sys :: IPPROTO_SCTP) ; # [doc = " Protocol corresponding to `UDPLITE`."] # [cfg (all (feature = "all" , any (target_os = "android" , target_os = "freebsd" , target_os = "fuchsia" , target_os = "linux" ,)))] pub const UDPLITE : Protocol = Protocol (sys :: IPPROTO_UDPLITE) ; # [doc = " Protocol corresponding to `DIVERT`."] # [cfg (all (feature = "all" , any (target_os = "freebsd" , target_os = "openbsd")))] pub const DIVERT : Protocol = Protocol (sys :: IPPROTO_DIVERT) ; }
    };
}

impl_59!();