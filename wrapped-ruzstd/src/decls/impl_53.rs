macro_rules! deps {
    () => {
        LiteralsSectionParseError!();
        DecompressBlockError!();
    };
}

macro_rules! impl_53 {
    () => {
        deps!();
        impl From < LiteralsSectionParseError > for DecompressBlockError { fn from (val : LiteralsSectionParseError) -> Self { Self :: LiteralsSectionParseError (val) } }
    };
}

impl_53!()