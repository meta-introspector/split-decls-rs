macro_rules! deps {
    () => {
        AsyncRead!();
        ReadBuf!();
    };
}

macro_rules! impl_106 {
    () => {
        deps!();
        impl < P > AsyncRead for Pin < P > where P : DerefMut , P :: Target : AsyncRead , { fn poll_read (self : Pin < & mut Self > , cx : & mut Context < '_ > , buf : & mut ReadBuf < '_ > ,) -> Poll < io :: Result < () > > { crate :: util :: pin_as_deref_mut (self) . poll_read (cx , buf) } }
    };
}

impl_106!();