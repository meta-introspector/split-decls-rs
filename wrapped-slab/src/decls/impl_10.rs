macro_rules! deps {
    () => {
        GetDisjointMutError!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl std :: error :: Error for GetDisjointMutError { }
    };
}

impl_10!()