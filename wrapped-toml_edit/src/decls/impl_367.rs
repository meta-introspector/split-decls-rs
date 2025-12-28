macro_rules! deps {
    () => {
        KeyValuePairs!();
        SerializeInlineTable!();
    };
}

macro_rules! impl_367 {
    () => {
        deps!();
        impl SerializeInlineTable { pub (crate) fn map (len : Option < usize >) -> Self { let mut items : crate :: table :: KeyValuePairs = Default :: default () ; let key = Default :: default () ; if let Some (len) = len { items . reserve (len) ; } Self { items , key } } }
    };
}

impl_367!();