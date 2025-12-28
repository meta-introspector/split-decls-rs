macro_rules! deps {
    () => {
        MaybeStorageLive!();
    };
}

macro_rules! impl_166 {
    () => {
        deps!();
        impl < 'a > MaybeStorageLive < 'a > { pub fn new (always_live_locals : Cow < 'a , DenseBitSet < Local > >) -> Self { MaybeStorageLive { always_live_locals } } }
    };
}

impl_166!();