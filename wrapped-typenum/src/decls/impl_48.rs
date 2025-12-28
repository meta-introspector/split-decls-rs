macro_rules! deps {
    () => {
        Z0!();
        Zero!();
    };
}

macro_rules! impl_48 {
    () => {
        deps!();
        impl Zero for Z0 { }
    };
}

impl_48!();