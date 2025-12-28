macro_rules! deps {
    () => {
        GetBitsError!();
        Error!();
    };
}

macro_rules! impl_7 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl std :: error :: Error for GetBitsError { }
    };
}

impl_7!();