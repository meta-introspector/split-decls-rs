macro_rules! deps {
    () => {
        Pu128!();
    };
}

macro_rules! impl_352 {
    () => {
        deps!();
        impl From < Pu128 > for u128 { # [inline] fn from (value : Pu128) -> Self { value . get () } }
    };
}

impl_352!();