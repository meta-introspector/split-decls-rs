macro_rules! deps {
    () => {
        Assume!();
        QueryContext!();
        MaybeTransmutableQuery!();
    };
}

macro_rules! impl_61 {
    () => {
        deps!();
        impl < L , C > MaybeTransmutableQuery < L , C > where C : QueryContext , { pub (crate) fn new (src : L , dst : L , assume : crate :: Assume , context : C) -> Self { Self { src , dst , assume , context } } }
    };
}

impl_61!();