macro_rules! deps {
    () => {
        BlockTypeError!();
        Error!();
    };
}

macro_rules! impl_43 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl std :: error :: Error for BlockTypeError { }
    };
}

impl_43!()