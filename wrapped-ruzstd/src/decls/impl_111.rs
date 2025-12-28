macro_rules! deps {
    () => {
        GetBitsError!();
        HuffmanTableError!();
    };
}

macro_rules! impl_111 {
    () => {
        deps!();
        impl From < GetBitsError > for HuffmanTableError { fn from (val : GetBitsError) -> Self { Self :: GetBitsError (val) } }
    };
}

impl_111!()