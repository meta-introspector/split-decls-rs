macro_rules! deps {
    () => {
        UTerm!();
        Zero!();
    };
}

macro_rules! impl_345 {
    () => {
        deps!();
        impl Zero for UTerm { }
    };
}

impl_345!()