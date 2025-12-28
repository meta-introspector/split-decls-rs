macro_rules! deps {
    () => {
        Assume!();
        Answer!();
    };
}

macro_rules! Representation {
    () => {
        deps!();
        trait Representation { fn is_transmutable (src : Self , dst : Self , assume : Assume) -> Answer < ! , ! > ; }
    };
}

Representation!();