macro_rules! deps {
    () => {
        AsyncWrite!();
    };
}

macro_rules! impl_124 {
    () => {
        deps!();
        impl AsyncWrite for io :: Cursor < & mut Vec < u8 > > { fn poll_write (mut self : Pin < & mut Self > , _ : & mut Context < '_ > , buf : & [u8] ,) -> Poll < io :: Result < usize > > { Poll :: Ready (io :: Write :: write (& mut * self , buf)) } fn poll_write_vectored (mut self : Pin < & mut Self > , _ : & mut Context < '_ > , bufs : & [IoSlice < '_ >] ,) -> Poll < io :: Result < usize > > { Poll :: Ready (io :: Write :: write_vectored (& mut * self , bufs)) } fn is_write_vectored (& self) -> bool { true } fn poll_flush (mut self : Pin < & mut Self > , _ : & mut Context < '_ >) -> Poll < io :: Result < () > > { Poll :: Ready (io :: Write :: flush (& mut * self)) } fn poll_shutdown (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < io :: Result < () > > { self . poll_flush (cx) } }
    };
}

impl_124!();