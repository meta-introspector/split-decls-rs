macro_rules! deps {
    () => {
        Sha256VarCore!();
    };
}

macro_rules! impl_3 {
    () => {
        deps!();
        impl BlockSizeUser for Sha256VarCore { type BlockSize = U64 ; }
    };
}

impl_3!()