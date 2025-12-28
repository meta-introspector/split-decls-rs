macro_rules! deps {
    () => {
        TryFromSliceError!();
    };
}

macro_rules! impl_38 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl std :: error :: Error for TryFromSliceError { }
    };
}

impl_38!();