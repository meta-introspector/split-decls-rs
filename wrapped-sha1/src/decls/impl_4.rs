macro_rules! deps {
    () => {
        Sha1Core!();
    };
}

macro_rules! impl_4 {
    () => {
        deps!();
        impl BlockSizeUser for Sha1Core { type BlockSize = U64 ; }
    };
}

impl_4!()