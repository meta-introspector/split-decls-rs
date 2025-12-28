macro_rules! deps {
    () => {
        Interner!();
        Binder!();
    };
}

macro_rules! impl_217 {
    () => {
        deps!();
        impl < I : Interner , T : Eq > Eq for Binder < I , T > { }
    };
}

impl_217!()