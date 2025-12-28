macro_rules! deps {
    () => {
        Array!();
        Value!();
        IntoIter!();
        Item!();
        ArrayIter!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        impl < 's > IntoIterator for & 's Array { type Item = & 's Value ; type IntoIter = ArrayIter < 's > ; fn into_iter (self) -> Self :: IntoIter { self . iter () } }
    };
}

impl_11!()