macro_rules! deps {
    () => {
        Value!();
        IntoIter!();
        Item!();
        Array!();
        ArrayIntoIter!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl IntoIterator for Array { type Item = Value ; type IntoIter = ArrayIntoIter ; fn into_iter (self) -> Self :: IntoIter { Box :: new (self . values . into_iter () . filter (| v | v . is_value ()) . map (| v | v . into_value () . unwrap ()) ,) } }
    };
}

impl_10!()