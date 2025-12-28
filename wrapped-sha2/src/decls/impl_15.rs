macro_rules! deps {
    () => {
        Sha512VarCore!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        impl BlockSizeUser for Sha512VarCore { type BlockSize = U128 ; }
    };
}

impl_15!();