macro_rules! deps {
    () => {
        GetBitsError!();
        HuffmanDecoderError!();
    };
}

macro_rules! impl_117 {
    () => {
        deps!();
        impl From < GetBitsError > for HuffmanDecoderError { fn from (val : GetBitsError) -> Self { Self :: GetBitsError (val) } }
    };
}

impl_117!();