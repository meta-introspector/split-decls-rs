macro_rules! deps {
    () => {
        Error!();
        SequencesHeaderParseError!();
    };
}

macro_rules! impl_98 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl std :: error :: Error for SequencesHeaderParseError { }
    };
}

impl_98!();