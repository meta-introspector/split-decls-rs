macro_rules! deps {
    () => {
        TaggedRef!();
        Tag!();
        Aligned!();
    };
}

macro_rules! impl_583 {
    () => {
        deps!();
        impl < P , T > Clone for TaggedRef < '_ , P , T > where P : Aligned + ? Sized , T : Tag , { # [inline] fn clone (& self) -> Self { * self } }
    };
}

impl_583!();