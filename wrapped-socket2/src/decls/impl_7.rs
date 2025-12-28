macro_rules! deps {
    () => {
        SockAddrStorage!();
    };
}

macro_rules! impl_7 {
    () => {
        deps!();
        impl std :: fmt :: Debug for SockAddrStorage { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { f . debug_struct ("sockaddr_storage") . field ("ss_family" , & self . storage . ss_family) . finish_non_exhaustive () } }
    };
}

impl_7!()