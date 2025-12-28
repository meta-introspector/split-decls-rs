macro_rules! deps {
    () => {
        Tlv!();
    };
}

macro_rules! impl_318 {
    () => {
        deps!();
        impl Tlv { # [inline] pub (crate) fn null () -> Self { Self (ptr :: null ()) } }
    };
}

impl_318!()