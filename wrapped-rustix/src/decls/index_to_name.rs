macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! index_to_name {
    () => {
        deps!();
        # [doc = " `ioctl(fd, SIOCGIFNAME, ifreq)`—Returns the interface name for a given"] # [doc = " index."] # [doc = ""] # [doc = " See the [module-level documentation] for information about `fd` usage."] # [doc = ""] # [doc = " See also [`index_to_name_inlined`] which does not require `alloc` feature."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [module-level documentation]: self"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man7/netdevice.7.html"] # [inline] # [doc (alias = "SIOCGIFNAME")] # [cfg (feature = "alloc")] # [cfg_attr (docsrs , doc (cfg (feature = "alloc")))] pub fn index_to_name < Fd : AsFd > (fd : Fd , index : u32) -> io :: Result < String > { let (len , ifrn_name) = crate :: backend :: net :: netdevice :: index_to_name (fd . as_fd () , index) ? ; core :: str :: from_utf8 (& ifrn_name [.. len]) . map_err (| _ | io :: Errno :: ILSEQ) . map (ToOwned :: to_owned) }
    };
}

index_to_name!()