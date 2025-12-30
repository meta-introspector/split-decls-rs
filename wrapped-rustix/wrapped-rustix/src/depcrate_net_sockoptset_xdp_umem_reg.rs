// Generated macro for set_xdp_umem_reg (function)
macro_rules! Depcrate_net_sockoptset_xdp_umem_reg {
() => {
// Module: crate::net::sockopt
// Provides: {"set_xdp_umem_reg"}
// Dependencies: {}
# [doc = " `setsockopt(fd, SOL_XDP, XDP_UMEM_REG, value)`"] # [doc = ""] # [doc = " On kernel versions only supporting v1, the flags are ignored."] # [doc = ""] # [doc = " # References"] # [doc = "   - [Linux]"] # [doc = ""] # [doc = " [Linux]: https://www.kernel.org/doc/html/next/networking/af_xdp.html#xdp-umem-reg-setsockopt"] # [cfg (target_os = "linux")] # [doc (alias = "XDP_UMEM_REG")] pub fn set_xdp_umem_reg < Fd : AsFd > (fd : Fd , value : XdpUmemReg) -> io :: Result < () > { backend :: net :: sockopt :: set_xdp_umem_reg (fd . as_fd () , value) }
};
}
