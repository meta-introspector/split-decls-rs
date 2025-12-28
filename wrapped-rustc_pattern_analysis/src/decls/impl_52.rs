macro_rules! deps {
    () => {
        PatCx!();
        PatOrWild!();
    };
}

macro_rules! impl_52 {
    () => {
        deps!();
        impl < 'p , Cx : PatCx > Copy for PatOrWild < 'p , Cx > { }
    };
}

impl_52!()