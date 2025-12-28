macro_rules! deps {
    () => {
        Datetime!();
        Time!();
    };
}

macro_rules! impl_12 {
    () => {
        deps!();
        impl From < Time > for Datetime { fn from (other : Time) -> Self { Self { date : None , time : Some (other) , offset : None , } } }
    };
}

impl_12!()