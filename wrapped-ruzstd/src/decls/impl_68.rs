macro_rules! deps {
    () => {
        DictionaryDecodeError!();
        HuffmanTableError!();
    };
}

macro_rules! impl_68 {
    () => {
        deps!();
        impl From < HuffmanTableError > for DictionaryDecodeError { fn from (val : HuffmanTableError) -> Self { Self :: HuffmanTableError (val) } }
    };
}

impl_68!()