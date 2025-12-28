macro_rules! deps {
    () => {
        AsyncSeek!();
    };
}

macro_rules! impl_115 {
    () => {
        deps!();
        impl < T : AsRef < [u8] > + Unpin > AsyncSeek for io :: Cursor < T > { fn start_seek (mut self : Pin < & mut Self > , pos : SeekFrom) -> io :: Result < () > { io :: Seek :: seek (& mut * self , pos) . map (drop) } fn poll_complete (self : Pin < & mut Self > , _ : & mut Context < '_ >) -> Poll < io :: Result < u64 > > { Poll :: Ready (Ok (self . get_mut () . position ())) } }
    };
}

impl_115!()