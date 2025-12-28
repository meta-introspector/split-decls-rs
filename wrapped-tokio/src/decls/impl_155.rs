macro_rules! deps {
    () => {
        ReadyFuture!();
    };
}

macro_rules! impl_155 {
    () => {
        deps!();
        impl sealed :: ToSocketAddrsPriv for SocketAddrV6 { type Iter = std :: option :: IntoIter < SocketAddr > ; type Future = ReadyFuture < Self :: Iter > ; fn to_socket_addrs (& self , _ : sealed :: Internal) -> Self :: Future { SocketAddr :: V6 (* self) . to_socket_addrs (sealed :: Internal) } }
    };
}

impl_155!()