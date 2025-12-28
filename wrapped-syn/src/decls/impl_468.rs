macro_rules! deps {
    () => {
        TokenMarker!();
    };
}

macro_rules! impl_468 {
    () => {
        deps!();
        impl < F : Copy + FnOnce (TokenMarker) -> T , T : Token > Sealed for F { }
    };
}

impl_468!();