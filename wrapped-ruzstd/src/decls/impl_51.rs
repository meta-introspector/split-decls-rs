macro_rules! deps {
    () => {
        Error!();
        DecompressBlockError!();
    };
}

macro_rules! impl_51 {
    () => {
        deps!();
        impl From < Error > for DecompressBlockError { fn from (val : Error) -> Self { Self :: BlockContentReadError (val) } }
    };
}

impl_51!();