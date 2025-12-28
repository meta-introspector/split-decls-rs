macro_rules! deps {
    () => {
        CowMut!();
    };
}

macro_rules! impl_28 {
    () => {
        deps!();
        impl < T > Deref for CowMut < '_ , T > { type Target = T ; fn deref (& self) -> & T { match self { CowMut :: BorrowedMut (borrowed) => borrowed , CowMut :: Owned (owned) => owned , } } }
    };
}

impl_28!();