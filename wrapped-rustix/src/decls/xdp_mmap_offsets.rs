macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! xdp_mmap_offsets {
    () => {
        deps!();
        # [doc = " `getsockopt(fd, SOL_XDP, XDP_MMAP_OFFSETS)`"] # [doc = ""] # [doc = " # References"] # [doc = "   - [Linux]"] # [doc = ""] # [doc = " [Linux]: https://www.kernel.org/doc/html/next/networking/af_xdp.html"] # [cfg (linux_raw_dep)] # [doc (alias = "XDP_MMAP_OFFSETS")] pub fn xdp_mmap_offsets < Fd : AsFd > (fd : Fd) -> io :: Result < XdpMmapOffsets > { backend :: net :: sockopt :: xdp_mmap_offsets (fd . as_fd ()) }
    };
}

xdp_mmap_offsets!();