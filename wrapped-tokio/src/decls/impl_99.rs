macro_rules! deps {
    () => {
        AsyncBufRead!();
    };
}

macro_rules! impl_99 {
    () => {
        deps!();
        impl AsyncBufRead for & [u8] { fn poll_fill_buf (self : Pin < & mut Self > , _cx : & mut Context < '_ >) -> Poll < io :: Result < & [u8] > > { Poll :: Ready (Ok (* self)) } fn consume (mut self : Pin < & mut Self > , amt : usize) { * self = & self [amt ..] ; } }
    };
}

impl_99!();