macro_rules! deps {
    () => {
        BlockHeaderReadError!();
        Error!();
    };
}

macro_rules! impl_39 {
    () => {
        deps!();
        impl From < Error > for BlockHeaderReadError { fn from (val : Error) -> Self { Self :: ReadError (val) } }
    };
}

impl_39!()