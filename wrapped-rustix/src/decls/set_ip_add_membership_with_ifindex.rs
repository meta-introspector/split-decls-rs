macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! set_ip_add_membership_with_ifindex {
    () => {
        deps!();
        # [doc = " `setsockopt(fd, IPPROTO_IP, IP_ADD_MEMBERSHIP, multiaddr, address,"] # [doc = " ifindex)`"] # [doc = ""] # [doc = " This is similar to [`set_ip_add_membership`] but additionally allows an"] # [doc = " `ifindex` value to be given."] # [doc = ""] # [doc = " See the [module-level documentation] for more."] # [doc = ""] # [doc = " [module-level documentation]: self#references-for-get_ip_-and-set_ip_-functions"] # [cfg (any (apple , freebsdlike , linux_like , target_os = "fuchsia" , target_os = "openbsd"))] # [inline] # [doc (alias = "IP_ADD_MEMBERSHIP")] pub fn set_ip_add_membership_with_ifindex < Fd : AsFd > (fd : Fd , multiaddr : & Ipv4Addr , address : & Ipv4Addr , ifindex : u32 ,) -> io :: Result < () > { backend :: net :: sockopt :: set_ip_add_membership_with_ifindex (fd . as_fd () , multiaddr , address , ifindex ,) }
    };
}

set_ip_add_membership_with_ifindex!();