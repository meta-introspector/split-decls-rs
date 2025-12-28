macro_rules! deps {
    () => {
        SocketAddrOpaque!();
        SocketAddrAny!();
        SocketAddrLen!();
        SocketAddrArg!();
    };
}

macro_rules! impl_630 {
    () => {
        deps!();
        unsafe impl SocketAddrArg for SocketAddrAny { unsafe fn with_sockaddr < R > (& self , f : impl FnOnce (* const SocketAddrOpaque , SocketAddrLen) -> R ,) -> R { f (self . as_ptr () . cast () , self . addr_len ()) } }
    };
}

impl_630!();