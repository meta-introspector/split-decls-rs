macro_rules! RawIpv6PathMtuDiscovery {
    () => {
        # [doc = " A type for holding raw integer IPv6 Path MTU Discovery options."] # [cfg (linux_kernel)] pub type RawIpv6PathMtuDiscovery = i32 ;
    };
}

RawIpv6PathMtuDiscovery!()