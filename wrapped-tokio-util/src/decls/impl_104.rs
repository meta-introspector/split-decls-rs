macro_rules! deps {
    () => {
        Either!();
    };
}

macro_rules! impl_104 {
    () => {
        deps!();
        impl < L , R > futures_core :: stream :: Stream for Either < L , R > where L : futures_core :: stream :: Stream , R : futures_core :: stream :: Stream < Item = L :: Item > , { type Item = L :: Item ; fn poll_next (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { delegate_call ! (self . poll_next (cx)) } }
    };
}

impl_104!();