macro_rules! deps {
    () => {
        HuffmanTableError!();
        FSEDecoderError!();
    };
}

macro_rules! impl_112 {
    () => {
        deps!();
        impl From < FSEDecoderError > for HuffmanTableError { fn from (val : FSEDecoderError) -> Self { Self :: FSEDecoderError (val) } }
    };
}

impl_112!()