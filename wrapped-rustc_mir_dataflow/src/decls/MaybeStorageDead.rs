macro_rules! MaybeStorageDead {
    () => {
        pub struct MaybeStorageDead < 'a > { always_live_locals : Cow < 'a , DenseBitSet < Local > > , }
    };
}

MaybeStorageDead!()