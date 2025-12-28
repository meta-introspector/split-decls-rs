macro_rules! deps {
    () => {
        ExpectedSpan!();
    };
}

macro_rules! impl_45 {
    () => {
        deps!();
        impl < I > From < I > for ExpectedSpan where I : Into < String > , { fn from (name : I) -> Self { ExpectedSpan :: default () . named (name) } }
    };
}

impl_45!()