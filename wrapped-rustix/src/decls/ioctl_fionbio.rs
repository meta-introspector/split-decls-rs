macro_rules! deps {
    () => {
        Result!();
        Setter!();
    };
}

macro_rules! ioctl_fionbio {
    () => {
        deps!();
        # [doc = " `ioctl(fd, FIONBIO, &value)`—Enables or disables non-blocking mode."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Winsock]"] # [doc = "  - [NetBSD]"] # [doc = "  - [OpenBSD]"] # [doc = ""] # [doc = " [Winsock]: https://docs.microsoft.com/en-us/windows/win32/winsock/winsock-ioctls#unix-ioctl-codes"] # [doc = " [NetBSD]: https://man.netbsd.org/ioctl.2#GENERIC%20IOCTLS"] # [doc = " [OpenBSD]: https://man.openbsd.org/ioctl.2#GENERIC_IOCTLS"] # [inline] # [doc (alias = "FIONBIO")] pub fn ioctl_fionbio < Fd : AsFd > (fd : Fd , value : bool) -> io :: Result < () > { unsafe { let ctl = ioctl :: Setter :: < { c :: FIONBIO } , c :: c_int > :: new (value . into ()) ; ioctl :: ioctl (fd , ctl) } }
    };
}

ioctl_fionbio!()