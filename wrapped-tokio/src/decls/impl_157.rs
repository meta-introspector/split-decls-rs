macro_rules! deps {
    () => {
        ReadyFuture!();
    };
}

macro_rules! impl_157 {
    () => {
        deps!();
        impl sealed :: ToSocketAddrsPriv for (IpAddr , u16) { type Iter = std :: option :: IntoIter < SocketAddr > ; type Future = ReadyFuture < Self :: Iter > ; fn to_socket_addrs (& self , _ : sealed :: Internal) -> Self :: Future { let iter = Some (SocketAddr :: from (* self)) . into_iter () ; future :: ready (Ok (iter)) } }
    };
}

impl_157!();