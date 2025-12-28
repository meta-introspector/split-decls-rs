macro_rules! deps {
    () => {
        Spanned!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        # [cfg (feature = "serde")] impl < T > Spanned < T > { pub (crate) const START_FIELD : & str = START_FIELD ; pub (crate) const END_FIELD : & str = END_FIELD ; pub (crate) const VALUE_FIELD : & str = VALUE_FIELD ; }
    };
}

impl_8!();