macro_rules! deps {
    () => {
        ReadyFuture!();
    };
}

macro_rules! impl_153 {
    () => {
        deps!();
        impl sealed :: ToSocketAddrsPriv for SocketAddrV4 { type Iter = std :: option :: IntoIter < SocketAddr > ; type Future = ReadyFuture < Self :: Iter > ; fn to_socket_addrs (& self , _ : sealed :: Internal) -> Self :: Future { SocketAddr :: V4 (* self) . to_socket_addrs (sealed :: Internal) } }
    };
}

impl_153!()