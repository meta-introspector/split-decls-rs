macro_rules! deps {
    () => {
        SocketAddrArg!();
        SocketAddrLen!();
        SocketAddrOpaque!();
    };
}

macro_rules! impl_559 {
    () => {
        deps!();
        unsafe impl SocketAddrArg for SocketAddrV6 { unsafe fn with_sockaddr < R > (& self , f : impl FnOnce (* const SocketAddrOpaque , SocketAddrLen) -> R ,) -> R { call_with_sockaddr (& encode_sockaddr_v6 (self) , f) } }
    };
}

impl_559!()