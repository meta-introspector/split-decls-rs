macro_rules! deps {
    () => {
        Aligned!();
        Tag!();
        TaggedRef!();
    };
}

macro_rules! impl_582 {
    () => {
        deps!();
        impl < P , T > Copy for TaggedRef < '_ , P , T > where P : Aligned + ? Sized , T : Tag , { }
    };
}

impl_582!();