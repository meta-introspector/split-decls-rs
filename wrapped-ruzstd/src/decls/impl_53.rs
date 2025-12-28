macro_rules! deps {
    () => {
        DecompressBlockError!();
        LiteralsSectionParseError!();
    };
}

macro_rules! impl_53 {
    () => {
        deps!();
        impl From < LiteralsSectionParseError > for DecompressBlockError { fn from (val : LiteralsSectionParseError) -> Self { Self :: LiteralsSectionParseError (val) } }
    };
}

impl_53!();