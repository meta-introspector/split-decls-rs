macro_rules! deps {
    () => {
        SocketAddrArg!();
        SocketAddrOpaque!();
        SocketAddrLen!();
    };
}

macro_rules! impl_558 {
    () => {
        deps!();
        unsafe impl SocketAddrArg for SocketAddrV4 { unsafe fn with_sockaddr < R > (& self , f : impl FnOnce (* const SocketAddrOpaque , SocketAddrLen) -> R ,) -> R { call_with_sockaddr (& encode_sockaddr_v4 (self) , f) } }
    };
}

impl_558!()