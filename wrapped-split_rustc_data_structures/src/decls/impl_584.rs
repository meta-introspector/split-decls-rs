macro_rules! deps {
    () => {
        Aligned!();
        TaggedRef!();
        Tag!();
    };
}

macro_rules! impl_584 {
    () => {
        deps!();
        impl < P , T > Deref for TaggedRef < '_ , P , T > where P : Aligned + ? Sized , T : Tag , { type Target = P ; # [inline] fn deref (& self) -> & Self :: Target { self . pointer () } }
    };
}

impl_584!();