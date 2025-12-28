macro_rules! deps {
    () => {
        Representation!();
        MaybeTransmutableQuery!();
        Assume!();
        Tree!();
        Answer!();
    };
}

macro_rules! impl_46 {
    () => {
        deps!();
        impl Representation for Tree { fn is_transmutable (src : Self , dst : Self , assume : Assume) -> Answer < ! , ! > { crate :: maybe_transmutable :: MaybeTransmutableQuery :: new (src , dst , assume , UltraMinimal :: default () ,) . answer () } }
    };
}

impl_46!()