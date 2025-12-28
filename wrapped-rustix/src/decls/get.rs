macro_rules! deps {
    () => {
        Timespec!();
        Result!();
        Event!();
    };
}

macro_rules! get {
    () => {
        deps!();
        # [doc = " `port_get(port, timeout)`—Gets an event from a port."] # [doc = ""] # [doc = " If an unsupported timeout is passed, this function fails with"] # [doc = " [`io::Errno::INVAL`]."] # [doc = ""] # [doc = " # References"] # [doc = "  - [OpenSolaris]"] # [doc = "  - [illumos]"] # [doc = ""] # [doc = " [OpenSolaris]: https://www.unix.com/man-page/opensolaris/3C/port_get/"] # [doc = " [illumos]: https://illumos.org/man/3C/port_get"] # [doc (alias = "port_get")] pub fn get < Fd : AsFd > (port : Fd , timeout : Option < & Timespec >) -> io :: Result < Event > { syscalls :: port_get (port . as_fd () , timeout) }
    };
}

get!()