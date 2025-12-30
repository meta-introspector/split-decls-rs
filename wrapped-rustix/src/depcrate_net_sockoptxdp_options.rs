// Generated macro for xdp_options (function)
macro_rules! Depcrate_net_sockoptxdp_options {
() => {
// Module: crate::net::sockopt
// Provides: {"xdp_options"}
// Dependencies: {}
# [doc = " `getsockopt(fd, SOL_XDP, XDP_OPTIONS)`"] # [doc = ""] # [doc = " # References"] # [doc = "   - [Linux]"] # [doc = ""] # [doc = " [Linux]: https://www.kernel.org/doc/html/next/networking/af_xdp.html#xdp-options-getsockopt"] # [cfg (target_os = "linux")] # [doc (alias = "XDP_OPTIONS")] pub fn xdp_options < Fd : AsFd > (fd : Fd) -> io :: Result < XdpOptionsFlags > { backend :: net :: sockopt :: xdp_options (fd . as_fd ()) }
};
}
