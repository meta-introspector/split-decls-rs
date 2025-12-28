macro_rules! deps {
    () => {
        AsyncSeek!();
    };
}

macro_rules! impl_114 {
    () => {
        deps!();
        impl < P > AsyncSeek for Pin < P > where P : DerefMut , P :: Target : AsyncSeek , { fn start_seek (self : Pin < & mut Self > , pos : SeekFrom) -> io :: Result < () > { crate :: util :: pin_as_deref_mut (self) . start_seek (pos) } fn poll_complete (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < io :: Result < u64 > > { crate :: util :: pin_as_deref_mut (self) . poll_complete (cx) } }
    };
}

impl_114!()