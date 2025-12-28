macro_rules! deps {
    () => {
        Z0!();
        Abs!();
    };
}

macro_rules! impl_301 {
    () => {
        deps!();
        impl Abs for Z0 { type Output = Z0 ; }
    };
}

impl_301!()