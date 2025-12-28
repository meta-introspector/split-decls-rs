macro_rules! deps {
    () => {
        DecompressLiteralsError!();
        GetBitsError!();
    };
}

macro_rules! impl_80 {
    () => {
        deps!();
        impl From < GetBitsError > for DecompressLiteralsError { fn from (val : GetBitsError) -> Self { Self :: GetBitsError (val) } }
    };
}

impl_80!()