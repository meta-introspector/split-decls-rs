macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! getn_query {
    () => {
        deps!();
        # [doc = " `port_getn(port, NULL, 0, NULL)`—Queries the number of events"] # [doc = " available from a port."] # [doc = ""] # [doc = " To retrieve the events, use [`getn`]."] # [doc = ""] # [doc = " # References"] # [doc = "  - [OpenSolaris]"] # [doc = "  - [illumos]"] # [doc = ""] # [doc = " [OpenSolaris]: https://www.unix.com/man-page/opensolaris/3C/port_getn/"] # [doc = " [illumos]: https://illumos.org/man/3C/port_getn"] # [doc (alias = "port_getn")] pub fn getn_query < Fd : AsFd > (port : Fd) -> io :: Result < u32 > { syscalls :: port_getn_query (port . as_fd ()) }
    };
}

getn_query!();