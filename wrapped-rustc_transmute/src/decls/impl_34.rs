macro_rules! deps {
    () => {
        Region!();
    };
}

macro_rules! impl_34 {
    () => {
        deps!();
        # [cfg (test)] impl Region for usize { }
    };
}

impl_34!()