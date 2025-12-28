macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! ip_tos {
    () => {
        deps!();
        # [doc = " `getsockopt(fd, IPPROTO_IP, IP_TOS)`"] # [doc = ""] # [doc = " See the [module-level documentation] for more."] # [doc = ""] # [doc = " [module-level documentation]: self#references-for-get_ip_-and-set_ip_-functions"] # [cfg (any (bsd , linux_like , target_os = "aix" , target_os = "fuchsia" , target_os = "haiku" , target_os = "nto" , target_env = "newlib"))] # [inline] # [doc (alias = "IP_TOS")] pub fn ip_tos < Fd : AsFd > (fd : Fd) -> io :: Result < u8 > { backend :: net :: sockopt :: ip_tos (fd . as_fd ()) }
    };
}

ip_tos!();