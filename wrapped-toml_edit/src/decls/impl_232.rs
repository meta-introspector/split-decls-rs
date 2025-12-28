macro_rules! deps {
    () => {
        Table!();
        Item!();
        IntoIter!();
    };
}

macro_rules! impl_232 {
    () => {
        deps!();
        impl IntoIterator for Table { type Item = (String , Item) ; type IntoIter = IntoIter ; fn into_iter (self) -> Self :: IntoIter { Box :: new (self . items . into_iter () . map (| (k , value) | (k . into () , value))) } }
    };
}

impl_232!()