macro_rules! deps {
    () => {
        TokenMarker!();
        Peek!();
    };
}

macro_rules! impl_465 {
    () => {
        deps!();
        impl < F : Copy + FnOnce (TokenMarker) -> T , T : Token > Peek for F { type Token = T ; }
    };
}

impl_465!();