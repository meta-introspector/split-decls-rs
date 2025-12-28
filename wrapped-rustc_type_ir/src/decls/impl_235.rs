macro_rules! deps {
    () => {
        EarlyBinder!();
        Interner!();
        TypeVisitable!();
    };
}

macro_rules! impl_235 {
    () => {
        deps!();
        # [doc = " For early binders, you should first call `instantiate` before using any visitors."] # [cfg (feature = "nightly")] impl < I : Interner , T > ! TypeVisitable < I > for ty :: EarlyBinder < I , T > { }
    };
}

impl_235!()