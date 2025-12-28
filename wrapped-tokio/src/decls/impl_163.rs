macro_rules! deps {
    () => {
        ReadyFuture!();
    };
}

macro_rules! impl_163 {
    () => {
        deps!();
        impl sealed :: ToSocketAddrsPriv for & [SocketAddr] { type Iter = std :: vec :: IntoIter < SocketAddr > ; type Future = ReadyFuture < Self :: Iter > ; fn to_socket_addrs (& self , _ : sealed :: Internal) -> Self :: Future { # [inline] fn slice_to_vec (addrs : & [SocketAddr]) -> Vec < SocketAddr > { addrs . to_vec () } let iter = slice_to_vec (self) . into_iter () ; future :: ready (Ok (iter)) } }
    };
}

impl_163!();