macro_rules! deps {
    () => {
        Assume!();
        QueryContext!();
    };
}

macro_rules! MaybeTransmutableQuery {
    () => {
        deps!();
        pub (crate) struct MaybeTransmutableQuery < L , C > where C : QueryContext , { src : L , dst : L , assume : crate :: Assume , context : C , }
    };
}

MaybeTransmutableQuery!();