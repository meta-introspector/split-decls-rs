macro_rules! deps {
    () => {
        Datetime!();
        Date!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        impl From < Date > for Datetime { fn from (other : Date) -> Self { Self { date : Some (other) , time : None , offset : None , } } }
    };
}

impl_11!()