macro_rules! deps {
    () => {
        Either!();
    };
}

macro_rules! impl_102 {
    () => {
        deps!();
        impl < L , R > AsyncSeek for Either < L , R > where L : AsyncSeek , R : AsyncSeek , { fn start_seek (self : Pin < & mut Self > , position : SeekFrom) -> Result < () > { delegate_call ! (self . start_seek (position)) } fn poll_complete (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Result < u64 > > { delegate_call ! (self . poll_complete (cx)) } }
    };
}

impl_102!();