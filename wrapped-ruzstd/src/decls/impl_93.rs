macro_rules! deps {
    () => {
        Error!();
        LiteralsSectionParseError!();
        GetBitsError!();
    };
}

macro_rules! impl_93 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl std :: error :: Error for LiteralsSectionParseError { fn source (& self) -> Option < & (dyn std :: error :: Error + 'static) > { match self { LiteralsSectionParseError :: GetBitsError (source) => Some (source) , _ => None , } } }
    };
}

impl_93!()