macro_rules! deps {
    () => {
        IntoIter!();
        InlineTable!();
        Item!();
        Value!();
        InlineTableIntoIter!();
    };
}

macro_rules! impl_96 {
    () => {
        deps!();
        impl IntoIterator for InlineTable { type Item = (String , Value) ; type IntoIter = InlineTableIntoIter ; fn into_iter (self) -> Self :: IntoIter { Box :: new (self . items . into_iter () . filter (| (_ , value) | value . is_value ()) . map (| (key , value) | (key . into () , value . into_value () . unwrap ())) ,) } }
    };
}

impl_96!()