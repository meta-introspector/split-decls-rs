macro_rules! deps {
    () => {
        Next!();
    };
}

macro_rules! SharedNext {
    () => {
        deps!();
        # [derive (Clone)] struct SharedNext (Arc < Next >) ;
    };
}

SharedNext!()