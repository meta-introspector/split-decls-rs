macro_rules! deps {
    () => {
        MatchArm!();
        PatCx!();
    };
}

macro_rules! impl_134 {
    () => {
        deps!();
        impl < 'p , Cx : PatCx > Copy for MatchArm < 'p , Cx > { }
    };
}

impl_134!()