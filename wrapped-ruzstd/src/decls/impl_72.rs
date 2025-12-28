macro_rules! deps {
    () => {
        DictionaryDecodeError!();
        FrameDecoderError!();
    };
}

macro_rules! impl_72 {
    () => {
        deps!();
        impl From < DictionaryDecodeError > for FrameDecoderError { fn from (val : DictionaryDecodeError) -> Self { Self :: DictionaryDecodeError (val) } }
    };
}

impl_72!();