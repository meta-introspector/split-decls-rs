macro_rules! deps {
    () => {
        MaybeStorageDead!();
    };
}

macro_rules! impl_169 {
    () => {
        deps!();
        impl < 'a > MaybeStorageDead < 'a > { pub fn new (always_live_locals : Cow < 'a , DenseBitSet < Local > >) -> Self { MaybeStorageDead { always_live_locals } } }
    };
}

impl_169!();