macro_rules! deps {
    () => {
        TaggedRef!();
        Tag!();
    };
}

macro_rules! impl_587 {
    () => {
        deps!();
        impl < P , T : Tag > Eq for TaggedRef < '_ , P , T > { }
    };
}

impl_587!();