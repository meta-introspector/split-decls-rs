macro_rules! deps {
    () => {
        InlineTable!();
        IntoIter!();
        InlineTableIter!();
        Value!();
        Item!();
    };
}

macro_rules! impl_97 {
    () => {
        deps!();
        impl < 's > IntoIterator for & 's InlineTable { type Item = (& 's str , & 's Value) ; type IntoIter = InlineTableIter < 's > ; fn into_iter (self) -> Self :: IntoIter { self . iter () } }
    };
}

impl_97!()