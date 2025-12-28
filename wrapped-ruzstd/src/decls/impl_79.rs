macro_rules! deps {
    () => {
        DecompressLiteralsError!();
        HuffmanDecoderError!();
    };
}

macro_rules! impl_79 {
    () => {
        deps!();
        impl From < HuffmanDecoderError > for DecompressLiteralsError { fn from (val : HuffmanDecoderError) -> Self { Self :: HuffmanDecoderError (val) } }
    };
}

impl_79!();