macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! set_ip_drop_membership {
    () => {
        deps!();
        # [doc = " `setsockopt(fd, IPPROTO_IP, IP_DROP_MEMBERSHIP, multiaddr, interface)`"] # [doc = ""] # [doc = " This is similar to [`set_ip_drop_membership`] but always sets `ifindex`"] # [doc = " value to zero."] # [doc = ""] # [doc = " See the [module-level documentation] for more."] # [doc = ""] # [doc = " [module-level documentation]: self#references-for-get_ip_-and-set_ip_-functions"] # [inline] # [doc (alias = "IP_DROP_MEMBERSHIP")] pub fn set_ip_drop_membership < Fd : AsFd > (fd : Fd , multiaddr : & Ipv4Addr , interface : & Ipv4Addr ,) -> io :: Result < () > { backend :: net :: sockopt :: set_ip_drop_membership (fd . as_fd () , multiaddr , interface) }
    };
}

set_ip_drop_membership!();