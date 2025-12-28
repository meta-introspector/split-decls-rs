macro_rules! deps {
    () => {
        Error!();
        BlockSizeError!();
    };
}

macro_rules! impl_46 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl std :: error :: Error for BlockSizeError { }
    };
}

impl_46!()