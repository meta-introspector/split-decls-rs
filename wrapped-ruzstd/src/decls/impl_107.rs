macro_rules! deps {
    () => {
        FSEDecoderError!();
        GetBitsError!();
    };
}

macro_rules! impl_107 {
    () => {
        deps!();
        impl From < GetBitsError > for FSEDecoderError { fn from (val : GetBitsError) -> Self { Self :: GetBitsError (val) } }
    };
}

impl_107!();