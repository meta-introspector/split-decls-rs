macro_rules! deps {
    () => {
        BlockHeaderReadError!();
        BlockTypeError!();
    };
}

macro_rules! impl_40 {
    () => {
        deps!();
        impl From < BlockTypeError > for BlockHeaderReadError { fn from (val : BlockTypeError) -> Self { Self :: BlockTypeError (val) } }
    };
}

impl_40!();