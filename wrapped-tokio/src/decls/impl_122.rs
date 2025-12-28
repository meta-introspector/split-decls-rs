macro_rules! deps {
    () => {
        AsyncWrite!();
    };
}

macro_rules! impl_122 {
    () => {
        deps!();
        impl AsyncWrite for Vec < u8 > { fn poll_write (self : Pin < & mut Self > , _cx : & mut Context < '_ > , buf : & [u8] ,) -> Poll < io :: Result < usize > > { self . get_mut () . extend_from_slice (buf) ; Poll :: Ready (Ok (buf . len ())) } fn poll_write_vectored (mut self : Pin < & mut Self > , _ : & mut Context < '_ > , bufs : & [IoSlice < '_ >] ,) -> Poll < io :: Result < usize > > { Poll :: Ready (io :: Write :: write_vectored (& mut * self , bufs)) } fn is_write_vectored (& self) -> bool { true } fn poll_flush (self : Pin < & mut Self > , _cx : & mut Context < '_ >) -> Poll < io :: Result < () > > { Poll :: Ready (Ok (())) } fn poll_shutdown (self : Pin < & mut Self > , _cx : & mut Context < '_ >) -> Poll < io :: Result < () > > { Poll :: Ready (Ok (())) } }
    };
}

impl_122!()