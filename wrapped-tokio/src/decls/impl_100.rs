macro_rules! deps {
    () => {
        AsyncBufRead!();
    };
}

macro_rules! impl_100 {
    () => {
        deps!();
        impl < T : AsRef < [u8] > + Unpin > AsyncBufRead for io :: Cursor < T > { fn poll_fill_buf (self : Pin < & mut Self > , _cx : & mut Context < '_ >) -> Poll < io :: Result < & [u8] > > { Poll :: Ready (io :: BufRead :: fill_buf (self . get_mut ())) } fn consume (self : Pin < & mut Self > , amt : usize) { io :: BufRead :: consume (self . get_mut () , amt) ; } }
    };
}

impl_100!();