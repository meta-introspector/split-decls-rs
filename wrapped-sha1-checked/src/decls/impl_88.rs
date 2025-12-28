macro_rules! deps {
    () => {
        Sha1!();
    };
}

macro_rules! impl_88 {
    () => {
        deps!();
        impl BlockSizeUser for Sha1 { type BlockSize = U64 ; }
    };
}

impl_88!()