macro_rules! deps {
    () => {
        ReadyFuture!();
    };
}

macro_rules! impl_161 {
    () => {
        deps!();
        impl sealed :: ToSocketAddrsPriv for (Ipv6Addr , u16) { type Iter = std :: option :: IntoIter < SocketAddr > ; type Future = ReadyFuture < Self :: Iter > ; fn to_socket_addrs (& self , _ : sealed :: Internal) -> Self :: Future { let (ip , port) = * self ; SocketAddrV6 :: new (ip , port , 0 , 0) . to_socket_addrs (sealed :: Internal) } }
    };
}

impl_161!()