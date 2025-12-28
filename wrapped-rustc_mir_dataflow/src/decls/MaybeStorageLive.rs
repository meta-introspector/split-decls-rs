macro_rules! MaybeStorageLive {
    () => {
        pub struct MaybeStorageLive < 'a > { always_live_locals : Cow < 'a , DenseBitSet < Local > > , }
    };
}

MaybeStorageLive!();