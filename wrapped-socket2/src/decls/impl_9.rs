macro_rules! deps {
    () => {
        Domain!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        impl Domain { # [doc = " Domain for IPv4 communication, corresponding to `AF_INET`."] pub const IPV4 : Domain = Domain (sys :: AF_INET) ; # [doc = " Domain for IPv6 communication, corresponding to `AF_INET6`."] pub const IPV6 : Domain = Domain (sys :: AF_INET6) ; # [doc = " Domain for Unix socket communication, corresponding to `AF_UNIX`."] pub const UNIX : Domain = Domain (sys :: AF_UNIX) ; # [doc = " Returns the correct domain for `address`."] pub const fn for_address (address : SocketAddr) -> Domain { match address { SocketAddr :: V4 (_) => Domain :: IPV4 , SocketAddr :: V6 (_) => Domain :: IPV6 , } } }
    };
}

impl_9!()