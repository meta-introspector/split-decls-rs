macro_rules! deps {
    () => {
        Timespec!();
    };
}

macro_rules! impl_1684 {
    () => {
        deps!();
        impl AddAssign for Timespec { fn add_assign (& mut self , rhs : Self) { * self = * self + rhs ; } }
    };
}

impl_1684!();