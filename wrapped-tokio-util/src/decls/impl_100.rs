macro_rules! deps {
    () => {
        Either!();
    };
}

macro_rules! impl_100 {
    () => {
        deps!();
        impl < L , R > AsyncRead for Either < L , R > where L : AsyncRead , R : AsyncRead , { fn poll_read (self : Pin < & mut Self > , cx : & mut Context < '_ > , buf : & mut ReadBuf < '_ > ,) -> Poll < Result < () > > { delegate_call ! (self . poll_read (cx , buf)) } }
    };
}

impl_100!()