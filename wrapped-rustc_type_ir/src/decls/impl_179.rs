macro_rules! deps {
    () => {
        Interner!();
        Goal!();
    };
}

macro_rules! impl_179 {
    () => {
        deps!();
        impl < I : Interner , P : Eq > Eq for Goal < I , P > { }
    };
}

impl_179!()