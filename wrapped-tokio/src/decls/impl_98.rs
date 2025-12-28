macro_rules! deps {
    () => {
        AsyncBufRead!();
    };
}

macro_rules! impl_98 {
    () => {
        deps!();
        impl < P > AsyncBufRead for Pin < P > where P : DerefMut , P :: Target : AsyncBufRead , { fn poll_fill_buf (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < io :: Result < & [u8] > > { crate :: util :: pin_as_deref_mut (self) . poll_fill_buf (cx) } fn consume (self : Pin < & mut Self > , amt : usize) { crate :: util :: pin_as_deref_mut (self) . consume (amt) ; } }
    };
}

impl_98!()