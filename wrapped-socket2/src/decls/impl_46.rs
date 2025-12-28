macro_rules! deps {
    () => {
        SockRef!();
    };
}

macro_rules! impl_46 {
    () => {
        deps!();
        impl fmt :: Debug for SockRef < '_ > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("SockRef") . field ("raw" , & self . socket . as_raw ()) . field ("local_addr" , & self . socket . local_addr () . ok ()) . field ("peer_addr" , & self . socket . peer_addr () . ok ()) . finish () } }
    };
}

impl_46!()