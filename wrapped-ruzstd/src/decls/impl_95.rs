macro_rules! deps {
    () => {
        LiteralsSectionParseError!();
        GetBitsError!();
    };
}

macro_rules! impl_95 {
    () => {
        deps!();
        impl From < GetBitsError > for LiteralsSectionParseError { fn from (val : GetBitsError) -> Self { Self :: GetBitsError (val) } }
    };
}

impl_95!()