macro_rules! deps {
    () => {
        BlockHeaderReadError!();
        BlockSizeError!();
    };
}

macro_rules! impl_41 {
    () => {
        deps!();
        impl From < BlockSizeError > for BlockHeaderReadError { fn from (val : BlockSizeError) -> Self { Self :: BlockSizeError (val) } }
    };
}

impl_41!();