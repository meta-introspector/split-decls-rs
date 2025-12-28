macro_rules! deps {
    () => {
        SerializeStructVariant!();
        SerializeInlineTable!();
    };
}

macro_rules! impl_374 {
    () => {
        deps!();
        impl SerializeStructVariant { pub (crate) fn struct_ (variant : & 'static str , len : usize) -> Self { Self { variant , inner : SerializeInlineTable :: map (Some (len)) , } } }
    };
}

impl_374!();