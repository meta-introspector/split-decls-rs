macro_rules! impl_149 {
    () => {
        impl < T > sealed :: ToSocketAddrsPriv for & T where T : sealed :: ToSocketAddrsPriv + ? Sized , { type Iter = T :: Iter ; type Future = T :: Future ; fn to_socket_addrs (& self , _ : sealed :: Internal) -> Self :: Future { (* * self) . to_socket_addrs (sealed :: Internal) } }
    };
}

impl_149!();