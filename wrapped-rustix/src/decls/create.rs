macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! create {
    () => {
        deps!();
        # [doc = " `port_create()`—Creates a new port."] # [doc = ""] # [doc = " # References"] # [doc = "  - [OpenSolaris]"] # [doc = "  - [illumos]"] # [doc = ""] # [doc = " [OpenSolaris]: https://www.unix.com/man-page/opensolaris/3C/port_create/"] # [doc = " [illumos]: https://illumos.org/man/3C/port_create"] # [doc (alias = "port_create")] pub fn create () -> io :: Result < OwnedFd > { syscalls :: port_create () }
    };
}

create!()