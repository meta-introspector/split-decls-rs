macro_rules! deps {
    () => {
        Same!();
    };
}

macro_rules! impl_299 {
    () => {
        deps!();
        impl < T > Same < T > for T { type Output = T ; }
    };
}

impl_299!()