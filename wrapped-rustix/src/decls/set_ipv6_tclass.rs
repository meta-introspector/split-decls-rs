macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! set_ipv6_tclass {
    () => {
        deps!();
        # [doc = " `setsockopt(fd, IPPROTO_IPV6, IPV6_TCLASS, value)`"] # [doc = ""] # [doc = " See the [module-level documentation] for more."] # [doc = ""] # [doc = " [module-level documentation]: self#references-for-get_ipv6_-and-set_ipv6_-functions"] # [cfg (not (any (solarish , windows , target_os = "espidf" , target_os = "haiku" , target_os = "horizon" , target_os = "redox" , target_os = "vita")))] # [inline] # [doc (alias = "IPV6_TCLASS")] pub fn set_ipv6_tclass < Fd : AsFd > (fd : Fd , value : u32) -> io :: Result < () > { backend :: net :: sockopt :: set_ipv6_tclass (fd . as_fd () , value) }
    };
}

set_ipv6_tclass!();