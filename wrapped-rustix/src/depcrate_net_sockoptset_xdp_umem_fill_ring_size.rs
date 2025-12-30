// Generated macro for set_xdp_umem_fill_ring_size (function)
macro_rules! Depcrate_net_sockoptset_xdp_umem_fill_ring_size {
() => {
// Module: crate::net::sockopt
// Provides: {"set_xdp_umem_fill_ring_size"}
// Dependencies: {}
# [doc = " `setsockopt(fd, SOL_XDP, XDP_UMEM_FILL_RING, value)`"] # [doc = ""] # [doc = " # References"] # [doc = "   - [Linux]"] # [doc = ""] # [doc = " [Linux]: https://www.kernel.org/doc/html/next/networking/af_xdp.html#xdp-rx-tx-umem-fill-umem-completion-ring-setsockopts"] # [cfg (target_os = "linux")] # [doc (alias = "XDP_UMEM_FILL_RING")] pub fn set_xdp_umem_fill_ring_size < Fd : AsFd > (fd : Fd , value : u32) -> io :: Result < () > { backend :: net :: sockopt :: set_xdp_umem_fill_ring_size (fd . as_fd () , value) }
};
}
