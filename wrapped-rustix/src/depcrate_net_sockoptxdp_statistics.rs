// Generated macro for xdp_statistics (function)
macro_rules! Depcrate_net_sockoptxdp_statistics {
() => {
// Module: crate::net::sockopt
// Provides: {"xdp_statistics"}
// Dependencies: {}
# [doc = " `getsockopt(fd, SOL_XDP, XDP_STATISTICS)`"] # [doc = ""] # [doc = " # References"] # [doc = "   - [Linux]"] # [doc = ""] # [doc = " [Linux]: https://www.kernel.org/doc/html/next/networking/af_xdp.html#xdp-statistics-getsockopt"] # [cfg (linux_raw_dep)] # [doc (alias = "XDP_STATISTICS")] pub fn xdp_statistics < Fd : AsFd > (fd : Fd) -> io :: Result < XdpStatistics > { backend :: net :: sockopt :: xdp_statistics (fd . as_fd ()) }
};
}
