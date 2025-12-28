macro_rules! deps {
    () => {
        Zero!();
        UTerm!();
    };
}

macro_rules! impl_345 {
    () => {
        deps!();
        impl Zero for UTerm { }
    };
}

impl_345!();