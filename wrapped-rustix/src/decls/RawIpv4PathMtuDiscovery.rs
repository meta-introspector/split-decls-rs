macro_rules! RawIpv4PathMtuDiscovery {
    () => {
        # [doc = " A type for holding raw integer IPv4 Path MTU Discovery options."] # [cfg (linux_kernel)] pub type RawIpv4PathMtuDiscovery = i32 ;
    };
}

RawIpv4PathMtuDiscovery!();