macro_rules! deps {
    () => {
        TaggedRef!();
        Tag!();
        Aligned!();
    };
}

macro_rules! impl_590 {
    () => {
        deps!();
        unsafe impl < P , T > Sync for TaggedRef < '_ , P , T > where P : Sync + Aligned + ? Sized , T : Sync + Tag , { }
    };
}

impl_590!();