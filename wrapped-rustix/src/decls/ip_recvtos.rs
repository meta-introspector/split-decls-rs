macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! ip_recvtos {
    () => {
        deps!();
        # [doc = " `getsockopt(fd, IPPROTO_IP, IP_RECVTOS)`"] # [doc = ""] # [doc = " See the [module-level documentation] for more."] # [doc = ""] # [doc = " [module-level documentation]: self#references-for-get_ip_-and-set_ip_-functions"] # [cfg (any (apple , linux_like , target_os = "cygwin" , target_os = "freebsd" , target_os = "fuchsia" ,))] # [inline] # [doc (alias = "IP_RECVTOS")] pub fn ip_recvtos < Fd : AsFd > (fd : Fd) -> io :: Result < bool > { backend :: net :: sockopt :: ip_recvtos (fd . as_fd ()) }
    };
}

ip_recvtos!()