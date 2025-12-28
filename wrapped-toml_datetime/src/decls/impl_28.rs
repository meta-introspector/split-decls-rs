macro_rules! deps {
    () => {
        DatetimeParseError!();
    };
}

macro_rules! impl_28 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl std :: error :: Error for DatetimeParseError { }
    };
}

impl_28!();