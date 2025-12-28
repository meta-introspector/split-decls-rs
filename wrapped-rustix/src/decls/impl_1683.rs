macro_rules! deps {
    () => {
        Timespec!();
    };
}

macro_rules! impl_1683 {
    () => {
        deps!();
        impl Add for Timespec { type Output = Self ; fn add (self , rhs : Self) -> Self { self . checked_add (rhs) . expect ("overflow when adding timespecs") } }
    };
}

impl_1683!();