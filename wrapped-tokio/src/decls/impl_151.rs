macro_rules! deps {
    () => {
        ReadyFuture!();
    };
}

macro_rules! impl_151 {
    () => {
        deps!();
        impl sealed :: ToSocketAddrsPriv for SocketAddr { type Iter = std :: option :: IntoIter < SocketAddr > ; type Future = ReadyFuture < Self :: Iter > ; fn to_socket_addrs (& self , _ : sealed :: Internal) -> Self :: Future { let iter = Some (* self) . into_iter () ; future :: ready (Ok (iter)) } }
    };
}

impl_151!()