macro_rules! CannotUseFpConv {
    () => {
        # [derive (Copy , Clone)] struct CannotUseFpConv ;
    };
}

CannotUseFpConv!()