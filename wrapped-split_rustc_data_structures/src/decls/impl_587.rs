macro_rules! deps {
    () => {
        Tag!();
        TaggedRef!();
    };
}

macro_rules! impl_587 {
    () => {
        deps!();
        impl < P , T : Tag > Eq for TaggedRef < '_ , P , T > { }
    };
}

impl_587!()