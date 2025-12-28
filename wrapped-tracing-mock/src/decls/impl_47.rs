macro_rules! deps {
    () => {
        ExpectedSpan!();
    };
}

macro_rules! impl_47 {
    () => {
        deps!();
        impl From < & ExpectedSpan > for ExpectedSpan { fn from (span : & ExpectedSpan) -> Self { span . clone () } }
    };
}

impl_47!()