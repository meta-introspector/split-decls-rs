macro_rules! MonoItemCollectionStrategy {
    () => {
        # [derive (PartialEq)] pub (crate) enum MonoItemCollectionStrategy { Eager , Lazy , }
    };
}

MonoItemCollectionStrategy!();