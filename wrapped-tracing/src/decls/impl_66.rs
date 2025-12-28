macro_rules! deps {
    () => {
        Span!();
    };
}

macro_rules! impl_66 {
    () => {
        deps!();
        impl < 'a > From < & 'a Span > for Option < & 'a Id > { fn from (span : & 'a Span) -> Self { span . inner . as_ref () . map (| inner | & inner . id) } }
    };
}

impl_66!();