macro_rules! deps {
    () => {
        CycleHeads!();
        CycleHead!();
    };
}

macro_rules! impl_58 {
    () => {
        deps!();
        impl IntoIterator for CycleHeads { type Item = CycleHead ; type IntoIter = < ThinVec < Self :: Item > as IntoIterator > :: IntoIter ; fn into_iter (self) -> Self :: IntoIter { self . 0 . into_iter () } }
    };
}

impl_58!();