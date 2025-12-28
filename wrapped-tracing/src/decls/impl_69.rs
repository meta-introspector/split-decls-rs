macro_rules! deps {
    () => {
        EnteredSpan!();
    };
}

macro_rules! impl_69 {
    () => {
        deps!();
        impl < 'a > From < & 'a EnteredSpan > for Option < & 'a Id > { fn from (span : & 'a EnteredSpan) -> Self { span . inner . as_ref () . map (| inner | & inner . id) } }
    };
}

impl_69!()