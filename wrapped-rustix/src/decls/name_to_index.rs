macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! name_to_index {
    () => {
        deps!();
        # [doc = " `ioctl(fd, SIOCGIFINDEX, ifreq)`—Returns the interface index for a given"] # [doc = " name."] # [doc = ""] # [doc = " See the [module-level documentation] for information about `fd` usage."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [module-level documentation]: self"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man7/netdevice.7.html"] # [inline] # [doc (alias = "SIOCGIFINDEX")] pub fn name_to_index < Fd : AsFd > (fd : Fd , if_name : & str) -> io :: Result < u32 > { crate :: backend :: net :: netdevice :: name_to_index (fd . as_fd () , if_name) }
    };
}

name_to_index!();