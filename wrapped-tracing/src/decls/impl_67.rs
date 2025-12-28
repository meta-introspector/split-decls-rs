macro_rules! deps {
    () => {
        Span!();
        Inner!();
    };
}

macro_rules! impl_67 {
    () => {
        deps!();
        impl < 'a > From < & 'a Span > for Option < Id > { fn from (span : & 'a Span) -> Self { span . inner . as_ref () . map (Inner :: id) } }
    };
}

impl_67!()