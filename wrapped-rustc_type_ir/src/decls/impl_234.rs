macro_rules! deps {
    () => {
        Interner!();
        TypeFoldable!();
        EarlyBinder!();
    };
}

macro_rules! impl_234 {
    () => {
        deps!();
        # [doc = " For early binders, you should first call `instantiate` before using any visitors."] # [cfg (feature = "nightly")] impl < I : Interner , T > ! TypeFoldable < I > for ty :: EarlyBinder < I , T > { }
    };
}

impl_234!()