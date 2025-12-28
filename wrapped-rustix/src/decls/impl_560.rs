macro_rules! deps {
    () => {
        SocketAddrArg!();
        SocketAddrOpaque!();
        SocketAddrLen!();
    };
}

macro_rules! impl_560 {
    () => {
        deps!();
        # [cfg (unix)] unsafe impl SocketAddrArg for SocketAddrUnix { unsafe fn with_sockaddr < R > (& self , f : impl FnOnce (* const SocketAddrOpaque , SocketAddrLen) -> R ,) -> R { f (as_ptr (& self . unix) . cast () , self . addr_len ()) } }
    };
}

impl_560!()