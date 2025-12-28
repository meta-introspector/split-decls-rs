macro_rules! deps {
    () => {
        Either!();
    };
}

macro_rules! impl_103 {
    () => {
        deps!();
        impl < L , R > AsyncWrite for Either < L , R > where L : AsyncWrite , R : AsyncWrite , { fn poll_write (self : Pin < & mut Self > , cx : & mut Context < '_ > , buf : & [u8]) -> Poll < Result < usize > > { delegate_call ! (self . poll_write (cx , buf)) } fn poll_flush (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < tokio :: io :: Result < () > > { delegate_call ! (self . poll_flush (cx)) } fn poll_shutdown (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < tokio :: io :: Result < () > > { delegate_call ! (self . poll_shutdown (cx)) } fn poll_write_vectored (self : Pin < & mut Self > , cx : & mut Context < '_ > , bufs : & [std :: io :: IoSlice < '_ >] ,) -> Poll < std :: result :: Result < usize , std :: io :: Error > > { delegate_call ! (self . poll_write_vectored (cx , bufs)) } fn is_write_vectored (& self) -> bool { match self { Self :: Left (l) => l . is_write_vectored () , Self :: Right (r) => r . is_write_vectored () , } } }
    };
}

impl_103!()