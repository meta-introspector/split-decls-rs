macro_rules! deps {
    () => {
        CowMut!();
    };
}

macro_rules! impl_29 {
    () => {
        deps!();
        impl < T > DerefMut for CowMut < '_ , T > { fn deref_mut (& mut self) -> & mut T { match self { CowMut :: BorrowedMut (borrowed) => borrowed , CowMut :: Owned (owned) => owned , } } }
    };
}

impl_29!()