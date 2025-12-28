macro_rules! deps {
    () => {
        Backtrace!();
    };
}

macro_rules! impl_26 {
    () => {
        deps!();
        # [cfg (not (feature = "debug"))] impl Backtrace { fn new () -> Option < Self > { None } }
    };
}

impl_26!()