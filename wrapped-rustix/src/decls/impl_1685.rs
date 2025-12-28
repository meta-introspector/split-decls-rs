macro_rules! deps {
    () => {
        Timespec!();
    };
}

macro_rules! impl_1685 {
    () => {
        deps!();
        impl Sub for Timespec { type Output = Self ; fn sub (self , rhs : Self) -> Self { self . checked_sub (rhs) . expect ("overflow when subtracting timespecs") } }
    };
}

impl_1685!()