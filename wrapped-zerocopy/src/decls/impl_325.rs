macro_rules! deps {
    () => {
        Reference!();
        Exclusive!();
    };
}

macro_rules! impl_325 {
    () => {
        deps!();
        impl Reference for Exclusive { }
    };
}

impl_325!();