macro_rules! deps {
    () => {
        Interner!();
        NestedNormalizationGoals!();
    };
}

macro_rules! impl_197 {
    () => {
        deps!();
        impl < I : Interner > NestedNormalizationGoals < I > { pub fn empty () -> Self { NestedNormalizationGoals (vec ! []) } pub fn is_empty (& self) -> bool { self . 0 . is_empty () } }
    };
}

impl_197!()