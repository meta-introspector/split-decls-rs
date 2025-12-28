macro_rules! deps {
    () => {
        Ipv4PathMtuDiscovery!();
        Result!();
    };
}

macro_rules! set_ip_mtu_discover {
    () => {
        deps!();
        # [doc = " `setsockopt(fd, IPPROTO_IP, IP_MTU_DISCOVER, value)`"] # [doc = ""] # [doc = " See the [module-level documentation] for more."] # [doc = ""] # [doc = " [module-level documentation]: self#references-for-get_ip_-and-set_ip_-functions"] # [cfg (linux_kernel)] # [inline] # [doc (alias = "IP_MTU_DISCOVER")] pub fn set_ip_mtu_discover < Fd : AsFd > (fd : Fd , value : Ipv4PathMtuDiscovery) -> io :: Result < () > { backend :: net :: sockopt :: set_ip_mtu_discover (fd . as_fd () , value) }
    };
}

set_ip_mtu_discover!();