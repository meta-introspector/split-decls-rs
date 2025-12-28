macro_rules! deps {
    () => {
        SocketAddrArg!();
        SocketAddrOpaque!();
        SocketAddrLen!();
    };
}

macro_rules! impl_557 {
    () => {
        deps!();
        unsafe impl SocketAddrArg for SocketAddr { unsafe fn with_sockaddr < R > (& self , f : impl FnOnce (* const SocketAddrOpaque , SocketAddrLen) -> R ,) -> R { match self { Self :: V4 (v4) => v4 . with_sockaddr (f) , Self :: V6 (v6) => v6 . with_sockaddr (f) , } } }
    };
}

impl_557!()