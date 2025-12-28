macro_rules! deps {
    () => {
        Styled!();
    };
}

macro_rules! impl_332 {
    () => {
        deps!();
        impl < D : std :: fmt :: Display > Styled < D > { pub (crate) fn new (display : D , style : Style) -> Self { Self { display , style } } }
    };
}

impl_332!();