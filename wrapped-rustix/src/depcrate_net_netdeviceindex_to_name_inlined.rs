// Generated macro for index_to_name_inlined (function)
macro_rules! Depcrate_net_netdeviceindex_to_name_inlined {
() => {
// Module: crate::net::netdevice
// Provides: {"index_to_name_inlined"}
// Dependencies: {}
# [doc = " `ioctl(fd, SIOCGIFNAME, ifreq)`—Returns the interface name for a given"] # [doc = " index."] # [doc = ""] # [doc = " See the [module-level documentation] for information about `fd` usage."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [module-level documentation]: self"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man7/netdevice.7.html"] # [inline] # [doc (alias = "SIOCGIFNAME")] pub fn index_to_name_inlined < Fd : AsFd > (fd : Fd , index : u32) -> io :: Result < InlinedName > { let (len , ifrn_name) = crate :: backend :: net :: netdevice :: index_to_name (fd . as_fd () , index) ? ; core :: str :: from_utf8 (& ifrn_name [.. len]) . map_err (| _ | io :: Errno :: ILSEQ) . map (| _ | InlinedName { len , name : ifrn_name , }) }
};
}
