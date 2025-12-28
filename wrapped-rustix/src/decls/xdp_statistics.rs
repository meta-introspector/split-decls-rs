macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! xdp_statistics {
    () => {
        deps!();
        # [doc = " `getsockopt(fd, SOL_XDP, XDP_STATISTICS)`"] # [doc = ""] # [doc = " # References"] # [doc = "   - [Linux]"] # [doc = ""] # [doc = " [Linux]: https://www.kernel.org/doc/html/next/networking/af_xdp.html#xdp-statistics-getsockopt"] # [cfg (linux_raw_dep)] # [doc (alias = "XDP_STATISTICS")] pub fn xdp_statistics < Fd : AsFd > (fd : Fd) -> io :: Result < XdpStatistics > { backend :: net :: sockopt :: xdp_statistics (fd . as_fd ()) }
    };
}

xdp_statistics!();