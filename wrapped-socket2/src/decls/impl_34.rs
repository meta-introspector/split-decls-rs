macro_rules! deps {
    () => {
        Socket!();
    };
}

macro_rules! impl_34 {
    () => {
        deps!();
        impl fmt :: Debug for Socket { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Socket") . field ("raw" , & self . as_raw ()) . field ("local_addr" , & self . local_addr () . ok ()) . field ("peer_addr" , & self . peer_addr () . ok ()) . finish () } }
    };
}

impl_34!();