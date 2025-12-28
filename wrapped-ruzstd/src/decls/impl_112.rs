macro_rules! deps {
    () => {
        FSEDecoderError!();
        HuffmanTableError!();
    };
}

macro_rules! impl_112 {
    () => {
        deps!();
        impl From < FSEDecoderError > for HuffmanTableError { fn from (val : FSEDecoderError) -> Self { Self :: FSEDecoderError (val) } }
    };
}

impl_112!();