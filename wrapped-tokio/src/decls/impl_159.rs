macro_rules! deps {
    () => {
        ReadyFuture!();
    };
}

macro_rules! impl_159 {
    () => {
        deps!();
        impl sealed :: ToSocketAddrsPriv for (Ipv4Addr , u16) { type Iter = std :: option :: IntoIter < SocketAddr > ; type Future = ReadyFuture < Self :: Iter > ; fn to_socket_addrs (& self , _ : sealed :: Internal) -> Self :: Future { let (ip , port) = * self ; SocketAddrV4 :: new (ip , port) . to_socket_addrs (sealed :: Internal) } }
    };
}

impl_159!()