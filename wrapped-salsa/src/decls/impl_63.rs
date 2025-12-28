macro_rules! deps {
    () => {
        CycleHeads!();
        CycleHeadsIterator!();
        CycleHead!();
    };
}

macro_rules! impl_63 {
    () => {
        deps!();
        impl < 'a > std :: iter :: IntoIterator for & 'a CycleHeads { type Item = & 'a CycleHead ; type IntoIter = CycleHeadsIterator < 'a > ; fn into_iter (self) -> Self :: IntoIter { self . iter () } }
    };
}

impl_63!();