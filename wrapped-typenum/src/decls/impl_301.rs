macro_rules! deps {
    () => {
        Abs!();
        Z0!();
    };
}

macro_rules! impl_301 {
    () => {
        deps!();
        impl Abs for Z0 { type Output = Z0 ; }
    };
}

impl_301!();