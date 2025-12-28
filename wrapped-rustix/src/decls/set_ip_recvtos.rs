macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! set_ip_recvtos {
    () => {
        deps!();
        # [doc = " `setsockopt(fd, IPPROTO_IP, IP_RECVTOS, value)`"] # [doc = ""] # [doc = " See the [module-level documentation] for more."] # [doc = ""] # [doc = " [module-level documentation]: self#references-for-get_ip_-and-set_ip_-functions"] # [cfg (any (apple , linux_like , target_os = "cygwin" , target_os = "freebsd" , target_os = "fuchsia" ,))] # [inline] # [doc (alias = "IP_RECVTOS")] pub fn set_ip_recvtos < Fd : AsFd > (fd : Fd , value : bool) -> io :: Result < () > { backend :: net :: sockopt :: set_ip_recvtos (fd . as_fd () , value) }
    };
}

set_ip_recvtos!();