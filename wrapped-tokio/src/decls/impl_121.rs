macro_rules! deps {
    () => {
        AsyncWrite!();
    };
}

macro_rules! impl_121 {
    () => {
        deps!();
        impl < P > AsyncWrite for Pin < P > where P : DerefMut , P :: Target : AsyncWrite , { fn poll_write (self : Pin < & mut Self > , cx : & mut Context < '_ > , buf : & [u8] ,) -> Poll < io :: Result < usize > > { crate :: util :: pin_as_deref_mut (self) . poll_write (cx , buf) } fn poll_write_vectored (self : Pin < & mut Self > , cx : & mut Context < '_ > , bufs : & [IoSlice < '_ >] ,) -> Poll < io :: Result < usize > > { crate :: util :: pin_as_deref_mut (self) . poll_write_vectored (cx , bufs) } fn is_write_vectored (& self) -> bool { (* * self) . is_write_vectored () } fn poll_flush (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < io :: Result < () > > { crate :: util :: pin_as_deref_mut (self) . poll_flush (cx) } fn poll_shutdown (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < io :: Result < () > > { crate :: util :: pin_as_deref_mut (self) . poll_shutdown (cx) } }
    };
}

impl_121!()