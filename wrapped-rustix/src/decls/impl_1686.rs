macro_rules! deps {
    () => {
        Timespec!();
    };
}

macro_rules! impl_1686 {
    () => {
        deps!();
        impl SubAssign for Timespec { fn sub_assign (& mut self , rhs : Self) { * self = * self - rhs ; } }
    };
}

impl_1686!();