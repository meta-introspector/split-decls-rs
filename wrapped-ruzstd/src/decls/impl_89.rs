macro_rules! deps {
    () => {
        DecodeSequenceError!();
        GetBitsError!();
    };
}

macro_rules! impl_89 {
    () => {
        deps!();
        impl From < GetBitsError > for DecodeSequenceError { fn from (val : GetBitsError) -> Self { Self :: GetBitsError (val) } }
    };
}

impl_89!()