macro_rules! deps {
    () => {
        FSEDecoderError!();
        DecodeSequenceError!();
    };
}

macro_rules! impl_91 {
    () => {
        deps!();
        impl From < FSEDecoderError > for DecodeSequenceError { fn from (val : FSEDecoderError) -> Self { Self :: FSEDecoderError (val) } }
    };
}

impl_91!();