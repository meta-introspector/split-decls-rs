macro_rules! deps {
    () => {
        ParseLevelFilterError!();
    };
}

macro_rules! impl_214 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl std :: error :: Error for ParseLevelFilterError { }
    };
}

impl_214!()