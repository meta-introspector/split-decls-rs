macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! openpt {
    () => {
        deps!();
        # [doc = " `posix_openpt(flags)`—Open a pseudoterminal device."] # [doc = ""] # [doc = " On Linux, an additional `CLOEXEC` flag value may be passed to request the"] # [doc = " close-on-exec flag be set."] # [doc = ""] # [doc = " On Linux, if the system has no free pseudoterminals available, the"] # [doc = " underlying system call fails with [`io::Errno::NOSPC`], however this rustix"] # [doc = " function translates that to [`io::Errno::AGAIN`], so that the linux_raw and"] # [doc = " libc backends have the same behavior."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = "  - [Apple]"] # [doc = "  - [FreeBSD]"] # [doc = "  - [DragonFly BSD]"] # [doc = "  - [NetBSD]"] # [doc = "  - [OpenBSD]"] # [doc = "  - [illumos]"] # [doc = ""] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/posix_openpt.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man3/posix_openpt.3.html"] # [doc = " [Apple]: https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man3/posix_openpt.3.html"] # [doc = " [FreeBSD]: https://man.freebsd.org/cgi/man.cgi?query=posix_openpt&sektion=2"] # [doc = " [DragonFly BSD]: https://man.dragonflybsd.org/?command=posix_openpt&section=3"] # [doc = " [NetBSD]: https://man.netbsd.org/posix_openpt.3"] # [doc = " [OpenBSD]: https://man.openbsd.org/posix_openpt"] # [doc = " [illumos]: https://illumos.org/man/3C/posix_openpt"] # [inline] # [doc (alias = "posix_openpt")] pub fn openpt (flags : OpenptFlags) -> io :: Result < OwnedFd > { # [cfg (linux_kernel)] { use crate :: fs :: { open , Mode } ; match open (cstr ! ("/dev/ptmx") , flags . into () , Mode :: empty ()) { Err (io :: Errno :: NOSPC) => Err (io :: Errno :: AGAIN) , otherwise => otherwise , } } # [cfg (not (linux_kernel))] { backend :: pty :: syscalls :: openpt (flags) } }
    };
}

openpt!()