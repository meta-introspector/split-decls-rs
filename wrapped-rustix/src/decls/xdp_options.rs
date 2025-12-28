macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! xdp_options {
    () => {
        deps!();
        # [doc = " `getsockopt(fd, SOL_XDP, XDP_OPTIONS)`"] # [doc = ""] # [doc = " # References"] # [doc = "   - [Linux]"] # [doc = ""] # [doc = " [Linux]: https://www.kernel.org/doc/html/next/networking/af_xdp.html#xdp-options-getsockopt"] # [cfg (target_os = "linux")] # [doc (alias = "XDP_OPTIONS")] pub fn xdp_options < Fd : AsFd > (fd : Fd) -> io :: Result < XdpOptionsFlags > { backend :: net :: sockopt :: xdp_options (fd . as_fd ()) }
    };
}

xdp_options!()