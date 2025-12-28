macro_rules! deps {
    () => {
        FSETableError!();
        GetBitsError!();
    };
}

macro_rules! impl_103 {
    () => {
        deps!();
        impl From < GetBitsError > for FSETableError { fn from (val : GetBitsError) -> Self { Self :: GetBitsError (val) } }
    };
}

impl_103!()