macro_rules! deps {
    () => {
        Tag!();
        Aligned!();
        TaggedRef!();
    };
}

macro_rules! impl_591 {
    () => {
        deps!();
        unsafe impl < P , T > Send for TaggedRef < '_ , P , T > where P : Sync + Aligned + ? Sized , T : Send + Tag , { }
    };
}

impl_591!();