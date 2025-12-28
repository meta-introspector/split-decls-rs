macro_rules! deps {
    () => {
        Type!();
    };
}

macro_rules! impl_35 {
    () => {
        deps!();
        # [cfg (test)] impl Type for () { }
    };
}

impl_35!()