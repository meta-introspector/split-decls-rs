macro_rules! deps {
    () => {
        Aligned!();
        TaggedRef!();
        Tag!();
    };
}

macro_rules! impl_583 {
    () => {
        deps!();
        impl < P , T > Clone for TaggedRef < '_ , P , T > where P : Aligned + ? Sized , T : Tag , { # [inline] fn clone (& self) -> Self { * self } }
    };
}

impl_583!()