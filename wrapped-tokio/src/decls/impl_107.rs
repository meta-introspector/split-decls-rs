macro_rules! deps {
    () => {
        ReadBuf!();
        AsyncRead!();
    };
}

macro_rules! impl_107 {
    () => {
        deps!();
        impl AsyncRead for & [u8] { fn poll_read (mut self : Pin < & mut Self > , _cx : & mut Context < '_ > , buf : & mut ReadBuf < '_ > ,) -> Poll < io :: Result < () > > { let amt = std :: cmp :: min (self . len () , buf . remaining ()) ; let (a , b) = self . split_at (amt) ; buf . put_slice (a) ; * self = b ; Poll :: Ready (Ok (())) } }
    };
}

impl_107!();