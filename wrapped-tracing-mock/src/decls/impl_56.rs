macro_rules! deps {
    () => {
        ExpectedSpan!();
        NewSpan!();
    };
}

macro_rules! impl_56 {
    () => {
        deps!();
        impl < S > From < S > for NewSpan where S : Into < ExpectedSpan > , { fn from (span : S) -> Self { Self { span : span . into () , .. Default :: default () } } }
    };
}

impl_56!()