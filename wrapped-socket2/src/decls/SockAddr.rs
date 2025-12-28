macro_rules! SockAddr {
    () => {
        # [doc = " The address of a socket."] # [doc = ""] # [doc = " `SockAddr`s may be constructed directly to and from the standard library"] # [doc = " [`SocketAddr`], [`SocketAddrV4`], and [`SocketAddrV6`] types."] # [derive (Clone)] pub struct SockAddr { storage : sockaddr_storage , len : socklen_t , }
    };
}

SockAddr!()