macro_rules! deps {
    () => {
        PatOrWild!();
        PatCx!();
    };
}

macro_rules! impl_52 {
    () => {
        deps!();
        impl < 'p , Cx : PatCx > Copy for PatOrWild < 'p , Cx > { }
    };
}

impl_52!();