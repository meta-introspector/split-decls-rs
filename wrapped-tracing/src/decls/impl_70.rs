macro_rules! deps {
    () => {
        Inner!();
        EnteredSpan!();
    };
}

macro_rules! impl_70 {
    () => {
        deps!();
        impl < 'a > From < & 'a EnteredSpan > for Option < Id > { fn from (span : & 'a EnteredSpan) -> Self { span . inner . as_ref () . map (Inner :: id) } }
    };
}

impl_70!();