macro_rules! deps {
    () => {
        TaggedRef!();
        Tag!();
        Aligned!();
    };
}

macro_rules! impl_582 {
    () => {
        deps!();
        impl < P , T > Copy for TaggedRef < '_ , P , T > where P : Aligned + ? Sized , T : Tag , { }
    };
}

impl_582!()