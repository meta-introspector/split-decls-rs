macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! preadv2 {
    () => {
        deps!();
        # [doc = " `preadv2(fd, bufs, offset, flags)`—Reads data, with several options."] # [doc = ""] # [doc = " An `offset` of `u64::MAX` means to use and update the current file offset."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux]"] # [doc = "  - [glibc]"] # [doc = ""] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/preadv2.2.html"] # [doc = " [glibc]: https://sourceware.org/glibc/manual/latest/html_node/Scatter_002dGather.html#index-preadv64v2"] # [cfg (all (linux_kernel , not (target_os = "android")))] # [inline] pub fn preadv2 < Fd : AsFd > (fd : Fd , bufs : & mut [IoSliceMut < '_ >] , offset : u64 , flags : ReadWriteFlags ,) -> io :: Result < usize > { backend :: io :: syscalls :: preadv2 (fd . as_fd () , bufs , offset , flags) }
    };
}

preadv2!()