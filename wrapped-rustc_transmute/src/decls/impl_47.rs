macro_rules! deps {
    () => {
        Representation!();
        Dfa!();
        MaybeTransmutableQuery!();
        Assume!();
        Answer!();
    };
}

macro_rules! impl_47 {
    () => {
        deps!();
        impl Representation for Dfa { fn is_transmutable (src : Self , dst : Self , assume : Assume) -> Answer < ! , ! > { crate :: maybe_transmutable :: MaybeTransmutableQuery :: new (src , dst , assume , UltraMinimal :: default () ,) . answer () } }
    };
}

impl_47!();