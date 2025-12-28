macro_rules! deps {
    () => {
        Inner!();
        Span!();
    };
}

macro_rules! impl_68 {
    () => {
        deps!();
        impl From < Span > for Option < Id > { fn from (span : Span) -> Self { span . inner . as_ref () . map (Inner :: id) } }
    };
}

impl_68!();