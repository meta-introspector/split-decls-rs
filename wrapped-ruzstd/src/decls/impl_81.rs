macro_rules! deps {
    () => {
        DecompressLiteralsError!();
        HuffmanTableError!();
    };
}

macro_rules! impl_81 {
    () => {
        deps!();
        impl From < HuffmanTableError > for DecompressLiteralsError { fn from (val : HuffmanTableError) -> Self { Self :: HuffmanTableError (val) } }
    };
}

impl_81!();