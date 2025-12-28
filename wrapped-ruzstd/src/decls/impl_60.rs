macro_rules! deps {
    () => {
        DecompressBlockError!();
        DecodeBlockContentError!();
    };
}

macro_rules! impl_60 {
    () => {
        deps!();
        impl From < DecompressBlockError > for DecodeBlockContentError { fn from (val : DecompressBlockError) -> Self { Self :: DecompressBlockError (val) } }
    };
}

impl_60!();