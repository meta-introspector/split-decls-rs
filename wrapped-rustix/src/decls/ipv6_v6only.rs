macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! ipv6_v6only {
    () => {
        deps!();
        # [doc = " `getsockopt(fd, IPPROTO_IPV6, IPV6_V6ONLY)`"] # [doc = ""] # [doc = " See the [module-level documentation] for more."] # [doc = ""] # [doc = " [module-level documentation]: self#references-for-get_ipv6_-and-set_ipv6_-functions"] # [inline] # [doc (alias = "IPV6_V6ONLY")] pub fn ipv6_v6only < Fd : AsFd > (fd : Fd) -> io :: Result < bool > { backend :: net :: sockopt :: ipv6_v6only (fd . as_fd ()) }
    };
}

ipv6_v6only!()