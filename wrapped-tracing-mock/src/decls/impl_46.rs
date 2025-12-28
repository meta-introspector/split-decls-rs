macro_rules! deps {
    () => {
        ExpectedId!();
        ExpectedSpan!();
    };
}

macro_rules! impl_46 {
    () => {
        deps!();
        impl From < & ExpectedId > for ExpectedSpan { fn from (id : & ExpectedId) -> Self { ExpectedSpan :: default () . with_id (id . clone ()) } }
    };
}

impl_46!();