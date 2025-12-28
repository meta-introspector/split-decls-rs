macro_rules! Backtrace {
    () => {
        # [cfg (not (feature = "debug"))] # [derive (Debug , Copy , Clone)] struct Backtrace ;
    };
}

Backtrace!()