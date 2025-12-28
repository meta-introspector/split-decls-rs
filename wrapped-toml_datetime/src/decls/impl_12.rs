macro_rules! deps {
    () => {
        Time!();
        Datetime!();
    };
}

macro_rules! impl_12 {
    () => {
        deps!();
        impl From < Time > for Datetime { fn from (other : Time) -> Self { Self { date : None , time : Some (other) , offset : None , } } }
    };
}

impl_12!();