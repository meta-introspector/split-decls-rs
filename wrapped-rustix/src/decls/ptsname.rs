macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! ptsname {
    () => {
        deps!();
        # [doc = " `ptsname(fd)`—Return the name of a pseudoterminal."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = "  - [illumos]"] # [doc = "  - [glibc]"] # [doc = ""] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/ptsname.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man3/ptsname.3.html"] # [doc = " [illumos]: https://www.illumos.org/man/3C/ptsname"] # [doc = " [glibc]: https://sourceware.org/glibc/manual/latest/html_node/Allocation.html#index-ptsname"] # [cfg (all (feature = "alloc" , any (apple , linux_like , target_os = "freebsd" , target_os = "fuchsia" , target_os = "illumos")))] # [inline] # [doc (alias = "ptsname_r")] # [cfg_attr (docsrs , doc (cfg (feature = "alloc")))] pub fn ptsname < Fd : AsFd , B : Into < Vec < u8 > > > (fd : Fd , reuse : B) -> io :: Result < CString > { backend :: pty :: syscalls :: ptsname (fd . as_fd () , reuse . into ()) }
    };
}

ptsname!();